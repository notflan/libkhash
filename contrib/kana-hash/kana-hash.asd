(asdf:defsystem :kana-hash
  :description "Kana Hashes"
  :author "Plum (boku@plum.moe)"
  :license "GPLv3"
  :version "1.0.0"
  :serial t
  :depends-on (:cffi)
  :Components ((:file "package")
               (:file "constants")
               (:file "ffi")
               (:file "kana-hash")))
