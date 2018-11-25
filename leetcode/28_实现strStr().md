# 28.实现strStr()

## 问题
实现`strStr()`函数。

给定一个`haystack` 字符串和一个 `needle` 字符串，在 `haystack` 字符串中找出 `needle` 字符串出现的第一个位置 (从0开始)。如果不存在，则返回`-1`。

```
示例 1:
输入: haystack = "hello", needle = "ll"
输出: 2

示例 2:
输入: haystack = "aaaaa", needle = "bba"
输出: -1
```

**说明:**
当`needle`是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。
对于本题而言，当`needle`是空字符串时我们应当返回 0 。这与C语言的`strstr()`以及 Java的`indexOf()`定义相符。

## 解答
就是判断一个字符串中是否存在子串needle，可以用KMP算法在`O(n)`时间完成。先用朴素算法来解决，时间复杂度为`O(n*n)`:
```python
class Solution:
    def strStr(self, haystack, needle):
        if needle is None or haystack is None:
            return -1
        i, j = 0, 0
        for i in range(0, len(haystack)-len(needle)+1):
            while j < len(needle) and haystack[i+j] == needle[j]:
                j += 1
            if j == len(needle):
                return i
            j = 0
        return -1
```

朴素算法很简单很暴力，也很容易理解，但是会发现做了很多多余的事情，主要是匹配失败后都会又从头开始，然而，前面的字符串已经扫描过了，我们实际上已经得到了应该返回到哪里的信息，而KMP算法就利用了这个。KMP算法先是构造一个表，这个表存储了每个位置匹配失败后应该返回到哪里，根据这个，就不会做很多多余的匹配了。

```python
class Solution(object):
    def strStr(self, haystack, needle):     
        if haystack == needle or not needle:
            return 0
        if not haystack or len(needle) > len(haystack):
            return -1
        table = self.getPartialTable(needle)
        j = 0
        for i in range(len(haystack)):
            while j > 0 and haystack[i] != needle[j]:
                j = table[j-1]
            if haystack[i] == needle[j]:
                j += 1
            if j == len(needle):
                return i - j + 1
        return -1

    def getPartialTable(self, needle):
        table = [0]
        for i in range(1, len(needle)):
            j = table[i-1]
            while j > 0 and needle[i] != needle[j]:
                j = table[j-1]
            if needle[i] == needle[j]:
                table.append(j+1)
            else:
                table.append(j)
        return table
```