(ns bowling.frame)

(defn start-game []
  (vector))

(defn add-turn-pins [v pin-1 pin-2]
  (conj v [pin-1 pin-2]))


(add-turn-pins (start-game) 7 2)