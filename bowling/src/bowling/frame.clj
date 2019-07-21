(ns bowling.frame)

(defn start-game []
  (vector))

(defn add-turn-pins [v pin-1 pin-2 & rst]
  (cond
    (>= (count v) 10) (throw (Exception. "Cannot play more than 10 turns"))
    (and (= (count v) 9) (or (= pin-1 10) (= 10 (+ pin-1 pin-2)))) (conj v [pin-1 pin-2 (first rst)])
    (and (= (count v) 9) (and (not (= pin-1 10)) (not (= 10 (+ pin-1 pin-2))))) (throw (Exception. "Cannot play 3 rounds in the tenth turn with open frame"))
    :else (conj v [pin-1 pin-2])))

(defn last-result [score]
  (let [first (first score) 
        second (second score) 
        third (last score)]
    (if (= 10 first)
      (str "X" " " second " " third)
      (if (= 10 (+ first second))
        (str first "/" third)
        (str first " " second)))))

(defn get-printable-result [v]
  (apply vector 
    (map-indexed
      (fn [idx score]
        (cond
          (and (= 9 idx) (= 3 (count score))) (last-result score)
          (= 10 (get score 0)) "X"
          (= 10 (+ (get score 0) (get score 1))) (str (get score 0) "/") 
          :else (str (get score 0) " " (get score 1)))) v)))