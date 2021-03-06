(defpackage treeleaves/tests/main
  (:use :cl
        :treeleaves
        :fiveam)
  (:documentation "Unit tests for Treeleaves")
  (:import-from #:treeleaves.models
                #:*database-table-types*
                )
  (:import-from #:treeleaves.format
                #:split-dir
                #:fmt
                #:format-tags
                #:format-args
                #:find-tables
                #:parse-search-args
                #:parse-database-args
                )
  (:import-from #:treeleaves.cli)
  (:import-from #:treeleaves)
  )
(in-package :treeleaves/tests/main)

; Imports
(require "treeleaves")
(require "fiveam")

; Helper functions
(defun list= (l1 l2 &key (test #'eql))
  "Check if two lists are equal to each other "
  (loop for i in l1
     for j in l2
     always (funcall test i j)))

(defun type-equal (t1 t2)
  "Check if two types are equal to each other"
  (and (subtypep t1 t2)
       (subtypep t2 t1)))

; Treeleaves Tests Suite
(def-suite treeleaves
  :description "Main Treeleaves Test Suite")

; Treeleaves Models Suite
(def-suite treeleaves.models
  :description "Treeleaves.Models Test Suite"
  :in treeleaves)
(in-suite treeleaves.models)

; Test the database lookup
(test test-database-table-types
      ; Index into the database table types
      (let ((result (gethash "document" *database-table-types*)))
        (is (not (null result))
            "result should not be null")
        (is (type-equal 'document result)
            "*database-table-types* should contain the document class: ~a" result)))

; Treeleaves Format Suite
(def-suite treeleaves.format
  :description "Treeleaves.Format Tests Suite"
  :in treeleaves)
(in-suite treeleaves.format)

; split-dir
(test test-split-dir
      (let ((result (split-dir "/home/user/")))
       (is (equal (list "" "home" "user" "") result)
       "split-dir should split the directory into a list: ~a" result)))

; format-tags
(test test-format-tags
      (let ((result (format-tags (list "Books" "Historical" "Fiction"))))
        (is (equal "Books Historical Fiction
" result))
        "format-tags should format tags into a string"))

; fmt
(test test-fmt
      (let ((result (fmt "Hello")))
        (is (equal "Hello" result))
        "fmt should format text into a string"))

; format-tags
(test test-format-args
      (let ((result (format-args (list "Books" "Historical" "Fiction"))))
        (is (equal "Books Historical Fiction" result))
        "format-args should format arguments into a string"))

; find-tables
(test test-find-tables-nil
      (let ((result
              (find-tables "document -f ./documents.sqlite -q :tags Books %")))
        (is (equal nil result))
        "find-tables should return nil when -t is not passed in: ~a" result))

(test test-find-tables-document
      (let ((result
              (find-tables "-t document -f ./documents.sqlite -q :tags Books %")))
        (is (equal "document " result))
        "find-tables should return the name of the database table: ~a" result))

(test test-find-tables-full
      (let ((result
              (find-tables "./bin/treeleaves -t document -f ./documents.sqlite -q \":tags\" \"Books\" %")))
        (is (equal "document " result))
        "find-tables should return the name of the database table: ~a" result))

; parse-search-args
(test test-parse-search-args-tags
      (let ((result (parse-search-args ":tags Books %")))
        (is (equal (list ":tags" "Books %") result))
        "parse-search-args should return the keyword and the search term from the arguments"
        ))

(test test-parse-search-args-full
      (let ((result (parse-search-args "document -f ./documents.sqlite -qa :tags Books %")))
        (is (equal (list ":tags" "Books %") result))
        "parse-search-args should return the keyword and the search term from the arguments"
        ))

; parse-database-args
(test test-parse-database-args
      (let ((result (parse-database-args "document -f ./documents.sqlite -qa :tags Books %")))
        (is (equal "./documents.sqlite" result)
        "parse-database-args should return the file path of the database from the arguments")))

; Treeleaves CLI Suite
(def-suite treeleaves.cli
  :description "Treeleaves.CLI Test Suite"
  :in treeleaves)
(in-suite treeleaves.format)

;(test test-this-should-fail
      ;(let ((result nil))
        ;(is (equal t result))
        ;"Manual test to ensure asdf is running the test suite"))

; Run test suite manually
(run! 'treeleaves)

; treeleaves.model
; find-docs
;(defparameter db "documents.sqlite")
;(defparameter tables (list 'document))
;(connect db)
;(ensure-tables tables)
;(find-docs :query "Books")

; query-db
; query-db-all
