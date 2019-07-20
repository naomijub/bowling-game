(ns bowling.frame-test
  (:require [clojure.test :refer :all]
            [bowling.frame :refer :all]))

(deftest frame-test
  (testing "frame is generated as empty vector"
    (is (vector? (start-game)))
    (is (= 0 (count (start-game)))))
  (testing "vector has new scores"
    (let [score-0 (start-game)
          score-1 (add-turn-pins score-0 7 2)
          score-2 (add-turn-pins score-1 10 0)
          score-3 (add-turn-pins score-2 7 3)]
      (is (= [[7 2]] score-1))
      (is (= [[7 2] [10 0]] score-2))
      (is (= [[7 2] [10 0] [7 3]] score-3)))))
