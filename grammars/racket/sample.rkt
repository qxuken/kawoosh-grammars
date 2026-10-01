#lang racket

;; A sample.
(require racket/list racket/match)

(provide area largest)

(define limit 10)

(struct point (x y) #:transparent)
(struct circle (radius))
(struct rect (width height))

(define (area shape)
  (match shape
    [(circle r) (* pi r r)]
    [(rect w h) #:when (> w 0) (* w h)]
    [_ 0]))

(define (largest items)
  (and (pair? items)
       (apply max items)))

(define-syntax-rule (twice body ...)
  (begin body ... body ...))

(define (point-length p)
  (sqrt (+ (sqr (point-x p)) (sqr (point-y p)))))

(module+ main
  (define shapes (list (circle 1.0) (rect 2.0 3.0)))
  (for ([s (in-list shapes)]
        [i (in-naturals)])
    (when (< (area s) limit)
      (printf "~a: ~a\n" i (area s))))
  (with-handlers ([exn:fail? (lambda (e) (displayln (exn-message e)))])
    (displayln (list (largest '(1 2 3))
                     (string->number "42")
                     (point-length (point 3 4))
                     #t #\a "text"))))
