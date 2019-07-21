(ns bowling.score-test
  (:require [clojure.test :refer :all]
            [bowling.score :refer :all]
            [bowling.frame :refer [start-game]]))

(deftest score-test
  (testing "empty vector is empty result"
    (is (= [] (get-scores (start-game)))))
  (testing "Different numeral interactions"
    (let [score-1 ["3 4"]
          score-2 ["3 4" "5 3"]
          score-3 ["3 4" "5 3" "2 6"]]
      (is (= [7] (get-scores score-1)))
      (is (= [7 8] (get-scores score-2)))
      (is (= [7 8 8] (get-scores score-3)))))
  (testing "with spare"
    (let [score-2 ["3/" "5 3"]
          score-3 ["3/" "5 3" "2 6"]
          score-4 ["3 0" "3/" "5 3"]]
      (is (= [15 8] (get-scores score-2)))
      (is (= [15 8 8] (get-scores score-3)))
      (is (= [3 15 8] (get-scores score-4)))))
  (testing "with strike"
    (let [score-1 ["X" "5 3"]
          score-2 ["X" "5/" "2 6"]
          score-3 ["X" "X" "5 3"]
          score-4 ["X" "X" "5/" "2 0"]
          score-5 ["X" "X" "X" "0 0"]]
      (is (= [18 8] (get-scores score-1)))
      (is (= [20 12 8] (get-scores score-2)))
      (is (= [25 18 8] (get-scores score-3)))
      (is (= [25 20 12 2] (get-scores score-4)))
      (is (= [30 20 10 0] (get-scores score-5))))))

(deftest score-types-test
  (testing "When score is all numeral returns the integer sum"
    (is (= 7 (sum-numeral-values "3 4")))
    (is (= 8 (sum-numeral-values "6 2"))))
  (testing "When the head item is a spare"
    (is (= 17 (sum-spare "3/" "7 2")))
    (is (= 13 (sum-spare "6/" "3/")))
    (is (= 20 (sum-spare "6/" "X")))
    (is (= 15 (sum-spare "6/5" nil))))
  (testing "When the head item is a strike"
    (testing "followed by spare"
      (is (= 20 (sum-strike "X" '("6/" "5 3")))))
    (testing "followed by simple value"
      (is (= 18 (sum-strike "X" '("6 2" "5 3")))))
    (testing "followed by strike"
      (is (= 25 (sum-strike "X" '("X" "5 3"))))
      (is (= 27 (sum-strike "X" '("X" "7/"))))
      (is (= 30 (sum-strike "X" '("X" "X")))))
    (testing "as last"
      (is (= 19 (sum-strike "X 4 5" '()))))))

(deftest final-score-test
  (testing "sum all scores"
    (let [scores [30 20 10 0 30 20 10 0 6 3]]
      (is (= 129 (get-final-score scores)))
      (is (= 171 (get-final-score (get-scores ["4 5" "X" "X" "7/" "7/" "7/" "7/" "7/" "7 5" "X 3 5"]))))
      (is (= 179 (get-final-score (get-scores ["4 5" "X" "X" "7/" "7/" "7/" "7/" "7/" "7/" "X 3 5"]))))
      (is (= 169 (get-final-score (get-scores ["4 5" "X" "X" "7/" "7/" "7/" "7/" "7/" "7 6" "6/5"]))))
      (is (= 179 (get-final-score (get-scores ["4 5" "X" "X" "7/" "7/" "7/" "7/" "7/" "X" "6/5"])))))))
