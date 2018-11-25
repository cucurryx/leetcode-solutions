# 52.N皇后 II

## 问题
`n`皇后问题研究的是如何将 `n`个皇后放置在 `n*n` 的棋盘上，并且使皇后彼此之间不能相互攻击。

![](https://leetcode-cn.com/static/images/problemset/8-queens.png)

上图为 8 皇后问题的一种解法。
给定一个整数 n，返回 n 皇后不同的解决方案的数量。

**示例**
```
输入: 4
输出: 2
解释: 4 皇后问题存在如下两个不同的解法。

[
[".Q..", // 解法 1
 "...Q",
 "Q...",
 "..Q."],
["..Q.", // 解法 2
 "Q...",
 "...Q",
 ".Q.."]
]
```

## 解答
经典题，可以不用递归来进行求解，可以用迭代来实现回溯法。主要的思路是，用一个`temp
`来记录每行棋子放置的位置，所以`temp[i]`应该初始化为0。然后对`k`行，我们先尝试放置棋子，循环结束后的可能性只要三种：成功放置并且已经满了，那么加入result，第二种就是成功放置，但是还没满，那么就尝试向下一行进行放置，最后一种就是放置失败，进行回溯。

```python
class Solution:
    def totalNQueens(self, n):
        k, result = 0, 0
        temp = [0]*n
        while k >= 0:
            while temp[k] < n and not self.isOk(temp, k, temp[k], n):
                temp[k] += 1
            if k == n-1 and temp[k] < n:
                result += 1
                temp[k] += 1
            elif temp[k] < n:
                k += 1
            else:
                k -= 1
                temp[k+1] = 0
                temp[k] = temp[k]+1
        return result

    def isOk(self, temp, row, column, n):
        for i in range(0, row):
            if column == temp[i] or abs(i-row) == abs(temp[i]-column):
                return False
        return True
```
