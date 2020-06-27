(in-package :kana-hash-ffi)

(define-foreign-library libkhash
  (:unix (:or "libkhash.so" "libkhash" "./libkhash.so" "/usr/lib/libkhash.so" "/usr/local/lib/libkhash.so"))
  (t (:default "libkhash")))

(use-foreign-library libkhash)

(defcstruct khash-salt
  (salt_type :char)
  (size :int)
  (body :pointer))

(defcstruct khash-ctx
  (algo :char)
  (flags :long)
  (khash_salt (:struct khash-salt)))

(defcfun "khash_max_length" :int
  (algo :char)
  (input-length :long)
  (digest-length :pointer))

(defcfun "khash_new_context" :int
  (algo :char)
  (salt_type :char)
  (data :pointer)
  (size :long)
  (output :pointer))

(defcfun "khash_free_context" :int
  (ctx :pointer))

(defcfun "khash_clone_context" :int
  (src :pointer)
  (dest :pointer))

(defcfun "khash_length" :int
  (context :pointer)
  (data :pointer)
  (size :long)
  (length :pointer))

(defcfun "khash_do" :int
  (context :pointer)
  (data :pointer)
  (size :long)
  (string :pointer)
  (strlen :long))

(defmacro with-khash-context (ctx &body body)
  `(with-foreign-object (,ctx '(:struct khash-ctx))
     ,@body))

(defmacro initialise-khash-context (context ctx &body body)
  `(with-foreign-object (,context '(:struct khash-ctx))
     (setf (mem-aref ,context '(:struct khash-ctx)) ,ctx)
     ,@body))

(defun get-length (string)
  (foreign-funcall "strlen" :pointer string :int))

(defmacro initialise-foreign-string (data _data &body body)
  `(with-foreign-string (data _data)
     (let ((len (get-length ,data)))
       ,@body)))

(defun ffi-khash-max-length (algo input-length)
  (with-foreign-object (digest-length :long)
    (values 
     (khash-max-length algo input-length digest-length)
     (mem-ref digest-length :long))))

(defun ffi-khash-new-context (algo salt-type _data)
  (with-khash-context output
    (with-foreign-string (data _data)
      (let ((len (get-length data)))
        (values
         (khash-new-context algo salt-type data len output)
         (mem-ref output '(:struct khash-ctx)))))))

(defun ffi-khash-free-context (ctx)
  (initialise-khash-context context ctx
    (khash-free-context context)))

(defun ffi-khash-clone-context (src)
  (with-khash-context destination
    (initialise-khash-context context src
      (khash-clone-context context destination)
      (mem-ref destination '(:struct khash-ctx)))))

(defun ffi-khash-length (ctx _data)
  (initialise-khash-context context ctx
    (with-foreign-string (data _data)
      (let ((len (get-length data)))
        (with-foreign-object (output-length :long)          
          (values (khash-length context data len output-length)
                  (mem-aref output-length :long)))))))

(defun ffi-khash-do (ctx _data length)
  (initialise-khash-context context ctx
    (with-foreign-string (data _data)
      (let ((len (get-length data)))
        (with-foreign-pointer (string (1+ length))
          (foreign-funcall "memset" :pointer string
                                    :int 0 
                                    :long (1+ length)
                                    :pointer)
          (values 
           (khash-do context data len string length)
           (foreign-string-to-lisp string)))))))

(export '(ffi-khash-do
          ffi-khash-length
          ffi-khash-clone-context
          ffi-khash-free-context
          ffi-khash-new-context
          ffi-khash-max-length))
