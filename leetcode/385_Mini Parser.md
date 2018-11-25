# 385.Mini Parser

## 问题
Given a nested list of integers represented as a string, implement a parser to deserialize it.

Each element is either an integer, or a list -- whose elements may also be integers or other lists.

Note:
You may assume that the string is well-formed:

- String is non-empty.
- String does not contain white spaces.
- String contains only digits 0-9, [, - ,, ].


```
Example 1:

Given s = "324",

You should return a NestedInteger object which contains a single integer 324.



Example 2:

Given s = "[123,[456,[789]]]",

Return a NestedInteger object containing a nested list with 2 elements:

1. An integer containing value 123.
2. A nested list containing two elements:
    i.  An integer containing value 456.
    ii. A nested list with one element:
         a. An integer containing value 789.
```


## 解答
简单的一个递归下降。比较坑的是，千万不要用题目给的`NestedInteger`，特别坑。实际上，python自带的list就可以通过测试了。

```python
class Solution:
    def deserialize(self, s):
        (result, _) = self.helper(s, 0)
        return result[0]

    def helper(self, s, start):
        result = list()
        i = start
        negative = False

        while i < len(s):
            if s[i] == '-':
                negative = True
                i += 1
            if s[i].isdigit():
                curr_num = 0
                while i < len(s) and s[i].isdigit():
                    curr_num *= 10
                    curr_num += int(s[i])
                    i += 1
                if negative:
                    curr_num = -curr_num
                    negative = False
                result.append(curr_num)
            elif s[i] == '[':
                (sub_result, new_start) = self.helper(s, i+1)
                result.append(sub_result)
                i = new_start
            elif s[i] == ']':
                break
            else:
                i += 1
        return (result, i+1)
```
