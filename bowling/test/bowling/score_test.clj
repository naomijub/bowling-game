(ns bowling.frame-test
  (:require [clojure.test :refer :all]
            [bowling.score :refer :all]
            [bowling.frame :refer [start-game]]))

(deftest score-test
  (testing "empty vector is empty result"
    (is (= [] (get-score (start-game)))))
  (testing "Different numeral interactions"
    (let [score-1 ["3 4"]
          score-2 ["3 4" "5 3"]
          score-3 ["3 4" "5 3" "2 6"]]
      (is (= [7] (get-score score-1)))
      (is (= [7 8] (get-score score-2)))
      (is (= [7 8 8] (get-score score-3)))))
  (testing "Testing with spae"
    (let [score-2 ["3/" "5 3"]
          score-3 ["3/" "5 3" "2 6"]
          score-4 ["3 0" "3/" "5 3"]]
      (is (= [15 8] (get-score score-2)))
      (is (= [15 8 8] (get-score score-3)))
      (is (= [3 15 8] (get-score score-4))))))

(deftest score-types-test
  (testing "When score is all numeral returns the integer sum"
    (is (= 7 (sum-numeral-values "3 4")))
    (is (= 8 (sum-numeral-values "6 2"))))
  (testing "When the head item is a spare"
    (is (= 17 (sum-spare "3/" "7 2")))
    (is (= 13 (sum-spare "6/" "3/")))))