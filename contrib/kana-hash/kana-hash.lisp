(in-package :kana-hash)

(define-condition kana-hash-error (error)
  ((msg :initarg :msg
        :reader messig))
  (:report (lambda (condition stream)
             (format stream "~a" (messig condition)))))

(defun handle-kana-hash-error (error-code)
  (error 'kana-hash-error :msg
         (cond
           ((= error-code KHASH_ERROR_IO) "IO Error")
           ((= error-code KHASH_ERROR_FORMAT) "Format Error")
           ((= error-code KHASH_ERROR_LENGTH) "Length Error")
           ((= error-code KHASH_ERROR_RNG) "RNG Error")
           ((= error-code KHASH_ERROR_UNKNOWN) "Unknown Error"))))

(defmacro with-success (thing function &body body)
  `(multiple-value-bind (return-code ,thing) ,function
     (if (= KHASH_SUCCESS return-code)
         (progn ,@body)
         (error 'kana-hash-error (handle-kana-hash-error return-code)))))


(defmacro with-context ((ctx algo salt-type salt) &body body)
  `(multiple-value-bind (return-code ,ctx) (ffi-khash-new-context ,algo ,salt-type ,salt)
     (unwind-protect          
          (progn
            (unless (= KHASH_SUCCESS return-code)
              (error 'kana-hash-error (handle-kana-hash-error return-code)))
            ,@body)
       (when ,ctx
         (ffi-khash-free-context ,ctx)))))

(defun make-hash (string &key (algo +algo-default+) (salt-type +salt-default+) salt
                  &aux (salt (or salt nil)))
  (when salt
    (setf salt-type +salt-specific+))
  (unless salt
    (setf salt (format nil "~a" salt)))
  (with-context (ctx algo salt-type salt)
    (with-success hash-length (ffi-khash-length ctx string)
      (with-success kana-hash (ffi-khash-do ctx string hash-length)
        (setf ctx nil)
        kana-hash))))

(export 'make-hash)
