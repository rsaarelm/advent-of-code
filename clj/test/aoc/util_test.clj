(ns aoc.util-test
  (:require [clojure.test :refer :all]
            [aoc.util :refer :all]))

(deftest chunks-test
  (is (= (text->chunks "") [""]))
  (is (= (text->chunks "%\n%\n%\n") ["" "" ""]))
  (is (= (text->chunks "a\nb") ["a" "b"]))
  (is (= (text->chunks "a\nb\n") ["a" "b"]))
  (is (= (text->chunks "a\nb\n%") ["a\nb"]))
  (is (= (text->chunks "a\n\n%\nb\n") ["a\n" "b"]))
  (is (= (text->chunks "a\n%\nb\n\n") ["a" "b\n"]))
  (is (= (text->chunks "a\n%\nb\n%\n") ["a" "b"]))
  (is (= (text->chunks "a\n%\nb\n") ["a" "b"])))

(deftest re-read-test
  (is (= (re-read "") ""))
  (is (= (re-read "cat") "cat"))
  (is (= (re-read "123") 123))
  (is (= (re-read "2.718") 2.718))
  (is (= (re-read #"(.+);(.+)" "xyzzy;123") ["xyzzy" 123]))
  (is (= (re-read [#"(foo)" #"(bar)(.+)"] "foo") ["foo"]))
  (is (= (re-read [#"(foo)" #"(bar)(.+)"] "bar666") ["bar" 666])))

(deftest sl-test
  (is (= (sl "") ""))
  (is (= (sl "abc") "abc"))
  (is (= (sl " abc") " abc"))
  (is (= (sl "one
              two
              three") "one\ntwo\nthree"))
  (is (= (sl "one

              two
              three") "one\n\ntwo\nthree")))
