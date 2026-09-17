# Greedy Algorithm

The greedy algorithm is an algorithm devised from the observation that `dynamic programming` often does too much work.

It doesn't replace dynamic programming, but is used alongside it, and they complement each other.

The greedy algorithm is also called the 'greedy' or 'selfish' algorithm.

It's a technique that doesn't consider the future and makes the best choice at each step.

It's an algorithm that hopes the best choice made at each step will also be the overall best.

Of course, the greedy algorithm doesn't work in all cases. For a simple example, in a problem where choosing now gets you 1 marshmallow, but waiting 1 minute and then choosing gets you 2 marshmallows, using a greedy algorithm would always result in only 1 marshmallow. This is because while the immediate best choice is to take 1 marshmallow, the overall best choice is to wait 1 minute and get 2.

## Activity Selection Problem
The activity selection problem, simply put, is about choosing the maximum number of classes that can be held simultaneously when trying to conduct multiple classes in one classroom.

Below, Si is the start time, and Fi is the finish time.

Since the same classroom must be used, A1 and A4 cannot be selected simultaneously.

A1 and A2 also overlap in time, so they cannot be selected.

However, A1 and A3 can be selected. The goal is to find the combination that allows the maximum number of classes to be held.

![](https://cdn.filepicker.io/api/file/MQKZ2QIHRie6hY1DDNGK)

Consequently, choosing a1, a3, a6, a8 or a1, a3, a7, a9, etc., would be correct. The problem is how to instruct a computer to make these selections.

While the problem can be solved using a DP approach, which requires calculating all possible combinations, using a **greedy algorithm** approach can solve it more efficiently.

### Problem Solving
Intuitively, to find the optimal solution, the first activity should finish as early as possible.

This is because it allows more other activities to be selected. In the case above, A1, which finishes earliest, should be chosen first. Once chosen, A2 and A4 can no longer be selected.

The next choice would be A3, which finishes next earliest. Then A6, followed by A8, resulting in A1, A3, A6, A8 as the final selection.

```js
var activity = [[1,1,3], [2,2,5], [3,4,7], [4,1,8], [5,5,9], [6,8,10], [7,9,11], [8,11,14], [9,13,16]];
function activitySelection(act) {
  var result = [];
  var sorted = act.sort(function(prev, cur) {
    return prev[2] - cur[2]; // 끝나는 시간 순으로 정렬
  });
  var last = 0;
  sorted.forEach(function(item) {
    if (last < item[1]) { // 조건 만족 시 결과 집합에 추가
      last = item[2];
      result.push(item);
    }
  });
  return result.map(function(r) {
    return r[0]; // map을 한 이유는 그냥 몇 번째 행동이 선택되었는지 보여주기 위함.
  });
}
activitySelection(activity); // [1, 3, 6, 8]
```
