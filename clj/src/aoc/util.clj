(ns aoc.util
  (:require [aoc.vec :as v]
            [clojure.string :as str]))

(defn- unfortune
  "Convert text in unix fortune format into separated text chunks.

  The separator is a line with a single '%' character.

  The last newline is omitted to allow newline-less single line inputs.
  To add a newline, leave an empty line before the separating '%'.

  The last item can be followed by an optional terminal '%'.

  BUGS
  - Any number of trailing newlines will be collapsed to
    a single newline at end."
  [lines]
  (->> lines
       ; Ignore terminal '%'.
       (#(if (= (peek %) "%") (pop %) %))
       (reduce
        (fn [acc line]
          (if (= line "%")
            (conj acc [])
            (conj (pop acc) (conj (peek acc) line))))
        [[]])
       (map (partial str/join "\n"))))

(defn text->chunks
  "Split multiline chunks if using fortune format, otherwise split lines."
  [text]
  (let
   [lines- (str/split-lines text)
       ; Hack around str/split-lines eating trailing newlines.
    lines (if (str/ends-with? text "\n\n") (conj lines- "") lines-)
    fortune-file? (some #{"%"} lines)]
    (if fortune-file? (unfortune lines) lines)))

(defn re-read
  "Tokenize a line with a regex and convert numbery results to numbers.

  When called without a regex,
  converts a single token to a number if it's numbery,
  otherwise returns it as is."
  ([str]
   (let
    [expr (if (empty? str) str (read-string str))]
     (if (number? expr) expr str)))
  ([regexps input]
   (let [match (fn [str]
                 (if (seqable? regexps)
                   (some #(re-find % str) regexps)
                   (re-find regexps str)))]
     (->> input
          (match)
          (rest)
          (mapv re-read)))))

(defn sl
  "Multiline string literal pretty-parsing.

    (sl \"these
         are
         the lines\")
    ; => \"these\\nare\\nthe lines\\n\""
  [s]
  (let
   [blank-prefix
    (fn [s]
      (loop [idx 0]
        (if s
          (if (and (< idx (.length s)) (Character/isWhitespace (.charAt s idx)))
            (recur (inc idx))
            (subs s 0 idx))
          "")))

    deindent
    (fn [line-seq]
      (let
       [prefix (blank-prefix (first (remove str/blank? line-seq)))
        deindent-line (fn [s]
                        (if (str/blank? s) s
                            (do
                              (when-not (str/starts-with? s prefix)
                                (throw (Error.
                                        "Line does not share first line's indentation")))
                              (subs s (count prefix)))))]
        (map deindent-line line-seq)))

    lines (str/split-lines s)]
    (str/join "\n" (cons (first lines) (deindent (rest lines))))))

; NB: str->grid is newer than sl
; and uses a slightly different uniform indent removal strategy.
; Leave an empty leading line with str->grid string literals.

(defn str->grid
  "Turn a string into `{[x y] -> char}` map.

  Whitespace does not become values.
  The string is clipped so that the first values
  (left-to-right, top-to-bottom) show up on x and y axes.
  This means that initial blank lines and indentation shared by all lines
  will be removed.
  Physical tabs will be treated as a runtime error
  when they cause situations where the x positions of subsequent characters
  cannot be reliably interpreted."
  [s]
  (let
   [count-tab #(get {\tab 1/65536} % 1)  ; Mark tab characters as fractions.
    row (fn [s]
          (->> (reductions (fn [acc c] (+ acc (count-tab c))) 0 s)
               (#(map vector % s))
               (filter (fn [[_ c]] (not (Character/isWhitespace c))))))
    raw-grid (->> (str/split-lines s)
                  (map-indexed
                   (fn [y line] (map (fn [[x c]] [[x y] c]) (row line))))
                  (apply concat))
    min-pos (->> (map first raw-grid) (reduce (partial mapv min)))
    ; Tab characters must be cancelled out in snapped-grid - all x coords
    ; become integers - or the result is invalid.
    snapped-grid (into {} (map (fn [[p c]] [(v/- p min-pos) c]) raw-grid))]
    (when (some (fn [[[x _] _]] (not (integer? x))) snapped-grid)
      (throw (Error. "Tab characters outside shared indent, cannot form grid.")))
    snapped-grid))

(defn points->box
  "Return smallest bounding box that contains all listed points.

  Outer edges are considered inclusive,
  you may want to add [1 1] to size if you want outer-exclusive box."
  [pts]
  (let
   [min-x (->> (map first pts) (reduce min))
    min-y (->> (map second pts) (reduce min))
    max-x (->> (map first pts) (reduce max))
    max-y (->> (map second pts) (reduce max))]
    [[min-x min-y] [(- max-x min-x) (- max-y min-y)]]))

(defn print-grid
  "Print a {[x y] -> char} map to stdout."
  [grid]
  (let
   [[[x y] [w h]] (points->box (keys grid))]
    (doseq [y (range y (+ y h 1))]
      (doseq [x (range x (+ x w 1))]
        (let [c (grid [x y])]
          (cond
            (char? c) (print c)
            :else (print \space))))
      (println))))
