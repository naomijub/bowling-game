(ns bowling.score)

(defn sum-numeral-values [n]
  (let [numbers (clojure.string/split n #" ")]
    (->> numbers
      (map #(Integer/parseInt %))
      (reduce +))))

(defn sum-spare [head next]
  (let [next-value (first (clojure.string/split next #""))]
    (+ 10 (Integer/parseInt next-value))))

(defn get-score [result] 
  (loop [results result scores []]
    (let [head (first results) tail (rest results)]
      (if (empty? results)
        scores
        (let [current-score 
              (cond 
                (clojure.string/includes? head "/") (conj scores (sum-spare head (first tail)))
                :else (conj scores (sum-numeral-values head)))]
          (recur tail current-score))))))
                   
