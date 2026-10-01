;;; sample.el --- A sample -*- lexical-binding: t; -*-

;;; Code:

(require 'cl-lib)

(defconst sample-limit 10
  "The limit.")

(defvar sample-shapes '((circle . 1.0) (rect . (2.0 3.0)))
  "Some shapes.")

(defun sample-area (shape)
  "The area of SHAPE."
  (pcase shape
    (`(circle . ,r) (* float-pi r r))
    (`(rect . (,w ,h)) (if (> w 0) (* w h) 0))
    (_ 0)))

(defun sample-largest (items)
  "The largest of ITEMS, or nil."
  (when items
    (cl-reduce #'max items)))

(defmacro sample-twice (&rest body)
  "Run BODY twice."
  `(progn ,@body ,@body))

;;;###autoload
(defun sample-run ()
  "Say what the shapes come to."
  (interactive)
  (let* ((areas (mapcar #'sample-area sample-shapes))
         (small (seq-filter (lambda (a) (< a sample-limit)) areas)))
    (dolist (a small)
      (message "%.2f" a))
    (condition-case err
        (message "%s %d" (sample-largest areas) (string-to-number "42"))
      (error (message "failed: %S" err)))))

(provide 'sample)
;;; sample.el ends here
