# 712.两个字符串的最小ASCII删除和

## 问题
给定两个字符串s1, s2，找到使两个字符串相等所需删除字符的ASCII值的最小和。

```
示例 1:

输入: s1 = "sea", s2 = "eat"
输出: 231
解释: 在 "sea" 中删除 "s" 并将 "s" 的值(115)加入总和。
在 "eat" 中删除 "t" 并将 116 加入总和。
结束时，两个字符串相等，115 + 116 = 231 就是符合条件的最小和。

示例`2:

输入: s1 = "delete", s2 = "leet"
输出: 403
解释: 在 "delete" 中删除 "dee" 字符串变成 "let"，
将 100[d]+101[e]+101[e] 加入总和。在 "leet" 中删除 "e" 将 101[e] 加入总和。
结束时，两个字符串都等于 "let"，结果即为 100+101+101+101 = 403 。
如果改为将两个字符串转换为 "lee" 或 "eet"，我们会得到 433 或 417 的结果，比答案更大。
```

**注意:**

- 0 < s1.length, s2.length <= 1000。
- 所有字符串中的字符ASCII值在[97, 122]之间。

## 解答
类似于[583_两个字符串的删除操作](./583_两个字符串的删除操作.md)。只不过在DP寻找最长子序列的时候，用ascii码的值来作为选择的标准。

```python
class Solution:
    def minimumDeleteSum(self, s1, s2):
        dp = [[0 for _ in range(len(s2)+1)] for __ in range(len(s1)+1)]
        for i in range(0, len(s1)):
            for j in range(0, len(s2)):
                if s1[i] == s2[j]:
                    dp[i+1][j+1] = ord(s1[i]) + dp[i][j]
                else:
                    dp[i+1][j+1] = max(dp[i][j+1], dp[i+1][j])
        return sum([ord(x) for x in s1]) + sum([ord(x) for x in s2]) - 2 * (dp[-1][-1])
```
