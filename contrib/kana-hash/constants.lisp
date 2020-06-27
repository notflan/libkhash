(in-package :kana-hash-ffi)
#.`(progn
     ,@(loop for (name . code) in
             '((KHASH_SUCCESS               . 0)
               (KHASH_ERROR_IO              . 1)
               (KHASH_ERROR_FORMAT          . 2)
               (KHASH_ERROR_LENGTH          . 3)
               (KHASH_ERROR_RNG             . 4)
               (KHASH_ERROR_UNKNOWN         . -1))
             collect `(defconstant ,name ,code)
             collect `(export (quote ,name))))


(in-package :kana-hash)
#.`(progn
     ,@(loop for (name . code) in
             '((+algo-default+          . 0)
               (+algo-crc32+            . 1)
               (+algo-crc64+            . 2)
               (+algo-sha256+           . 3)
               (+algo-sha256-truncated+ . 4)
               
               (+salt-none+     . 0)
               (+salt-default+  . 1)
               (+salt-specific+ . 2)
               (+salt-random+   . 3))
             collect `(defconstant ,name ,code)
             collect `(export (quote ,name))))
