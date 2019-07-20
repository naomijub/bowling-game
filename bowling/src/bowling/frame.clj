(ns bowling.frame)

(defn start-game []
  (vector))

(defn add-turn-pins [v pin-1 pin-2]
  (conj v [pin-1 pin-2]))

(defn get-printable-result [v]
  (mapv 
    (fn [score]
      (cond 
        (= 10 (get score 0)) "X"
        (= 10 (+ (get score 0) (get score 1))) (str (get score 0) "/") 
        :else (str (get score 0) " " (get score 1)))) v))  

