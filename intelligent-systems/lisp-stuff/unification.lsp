; This code is heavily based on the book.
;
; Artificial Intelligence:
;   Structures and strategies for complex problem solving
; by George F. Luger and William A. Stubblefield

; We define variables as lists where the first item is 'var, and the second the
; variable name.
;
; This function returns whether a pattern is a variable.
(defun is-var (pat)
  (and (listp pat)
       (equal (length pat) 2)
       (equal (car pat) 'var)))

; This function receives two patterns, the first of which is a variable, and
; then tries to match it, potentially adding a substitution to `substitutions`.
(defun match (var pattern substitutions)
  (assert (is-var var))
  (cond
    ; If they're equal we're done.
    ((equal var pattern) substitutions)
    ; Otherwise try to get the current value of `var` from `substitutions`.
    (T
      (let ((binding (assoc (car (cdr var)) substitutions)))
        (cond
          ; If the `var` was already matched, then unify it with `pattern`.
          (binding (unify (cdr binding) pattern substitutions))
          ; TODO(emilio): Should we check for recursive variable definitions
          ; and fail?
          ;
          ; Otherwise set `var -> pattern` in `substitutions`.
          (T (acons (car (cdr var)) pattern substitutions)))))))

(defun unify (pat1 pat2 &optional substitutions)
  (cond
    ; If we've already failed, we're doomed, can't unify.
    ((equal substitutions 'failure) 'failure)
    ; Try to get variables and match them.
    ((is-var pat1) (match pat1 pat2 substitutions))
    ((is-var pat2) (match pat2 pat1 substitutions))
    ; Or maybe both are constants, in which case we need to check whether
    ; they're the same (in which case they just match), or not, in which case
    ; we just fail.
    ((atom pat1)
       (cond
         ((equal pat1 pat2) substitutions)
         (T 'failure)))
    ; We know that pat1 is not a constant or a variable, but pat2 is a
    ; constant, so we can't substitute them.
    ((atom pat2) 'failure)
    ; The general case (with two lists), we try to unify the first two parts
    ; and then the rest, this is nicer than unifying them afterwards.
    (T (unify (cdr pat1) (cdr pat2) (unify (car pat1) (car pat2) substitutions)))))

; Tests
(assert (equal (match '(var x) '(var y) '((x . (var y)))) '((x . (var y)))))
(assert (equal (match '(var x) '(var y) '((x . 42))) '((y . 42) (x . 42))))
; Trivial case: unify(42, 43) -> should fail
(assert (equal (unify 42 43) 'failure))
; Trivial case: unify(42, 42)
(assert (equal (unify 42 42) nil))
; Trivial case: unify(x, x)
(assert (equal (unify '(var x) '(var x)) nil))
; Easy case: unify(42, x) -> { x: 42 }
(assert (equal (unify 42 '(var x)) '((x . 42))))
; Easy case: unify(42, x) -> { x: 42 }
(assert (equal (unify 42 '(var x)) '((x . 42))))
; Easy case: unify(x, y) -> { x: y }
(assert (equal (unify '(var x) '(var y)) '((x . (var y)))))
