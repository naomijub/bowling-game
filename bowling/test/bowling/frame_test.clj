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
      (is (= [[7 2] [10 0] [7 3]] score-3))))
  (testing "Results are obtained for strike and spare"
    (let [score [[7 2] [10 0] [7 3] [5 4]]
          expected ["7 2" "X" "7/" "5 4"]]
      (is (= expected (get-printable-result score)))
      (is (= (count expected) (count score)))))
  (testing "Exception is thrown when try to add extra results after 10 turns"
    (let [scores [[7 2] [10 0] [7 3] [7 2] [10 0] [7 3] [7 2] [10 0] [7 3] [3 4]]]
      (is (thrown? Exception (add-turn-pins scores 4 5))))))

