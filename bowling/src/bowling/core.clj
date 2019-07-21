(ns bowling.core
  (:require [bowling.frame :as f]
            [bowling.score :as s]))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [score (f/start-game)
        score-1 (f/add-turn-pins score 4 5)
        score-2 (f/add-turn-pins score-1 4 5)
        score-3 (f/add-turn-pins score-2 4 5)
        score-4 (f/add-turn-pins score-3 5 5)
        score-5 (f/add-turn-pins score-4 4 6)
        score-6 (f/add-turn-pins score-5 10 0)
        score-7 (f/add-turn-pins score-6 4 5)
        score-8 (f/add-turn-pins score-7 4 5)
        score-9 (f/add-turn-pins score-8 4 5)
        score-10 (f/add-turn-pins score-9 10 5 4)
        result (f/get-printable-result score-10)
        scores (s/get-scores result)]
    (do
      (println (str "Result: " result))
      (println (str "Frame Score: " scores))
      (println (str "Running Total: " (s/running-total scores)))
      (println (str "Final Score:" (s/get-final-score scores))))))
