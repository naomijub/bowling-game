(ns bowling.score)

(defn parse-first-int [i rgx]
  (Integer/parseInt (first (clojure.string/split i rgx))))

(defn sum-numeral-values [n]
  (let [numbers (clojure.string/split n #" ")]
    (->> numbers
      (map #(Integer/parseInt %))
      (reduce +))))

(defn sum-spare [head next]
  (if (nil? next)
    (+ 10 (Integer/parseInt (last (clojure.string/split head #""))))
    (if (clojure.string/includes? next  "X")
      20
      (+ 10 (parse-first-int next #"")))))

(defn last-strike-value [last]
  (let [item (first last)]
    (cond 
      (= "X" item) 10
      (clojure.string/includes? item  "/") (parse-first-int item #"")
      :else (parse-first-int item #" "))))

(defn sum-strike [head next-two]
  (if (empty? next-two)
    (->> (rest (clojure.string/split head #" "))
      (map #(Integer/parseInt %))
      (reduce + 10))
    (let [next (first next-two)]
      (cond
          (clojure.string/includes? next  "/") 20
          (clojure.string/includes? next  "X") (+ 20 (last-strike-value (rest next-two)))
          :else (+ 10 (sum-numeral-values (first next-two)))))))

(defn get-scores [result]
  (loop [results result scores []]
    (let [head (first results) tail (rest results)]
      (if (empty? results)
        scores          
        (let [current-score 
              (cond 
                (clojure.string/includes? head "X") (conj scores (sum-strike head (take 2 tail)))
                (clojure.string/includes? head "/") (conj scores (sum-spare head (first tail)))
                :else (conj scores (sum-numeral-values head)))]
          (recur tail current-score))))))
                   
(defn get-final-score [scores]
  (reduce + 0 scores))

(defn running-total [scores]
  (loop [head (first scores) tail (rest scores) total []]
    (let [new-head (+ head (first tail))
          new-tail (rest tail)
          new-total (conj total head)]
      (if (empty? new-tail)
        (conj new-total new-head)
        (recur new-head new-tail new-total)))))
