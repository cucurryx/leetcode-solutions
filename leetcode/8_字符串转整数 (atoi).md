# 8.字符串转整数 (atoi)

## 问题
实现 atoi，将字符串转为整数。

在找到第一个非空字符之前，需要移除掉字符串中的空格字符。如果第一个非空字符是正号或负号，选取该符号，并将其与后面尽可能多的连续的数字组合起来，这部分字符即为整数的值。如果第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成整数。

字符串可以在形成整数的字符后面包括多余的字符，这些字符可以被忽略，它们对于函数没有影响。

当字符串中的第一个非空字符序列不是个有效的整数；或字符串为空；或字符串仅包含空白字符时，则不进行转换。
若函数不能执行有效的转换，返回 0。

说明：
假设我们的环境只能存储 32 位有符号整数，其数值范围是[`-2^31`, `2^31 - 1`]。如果数值超过可表示的范围，则返回 `INT_MAX` 或`INT_MIN`。

```
示例 1:
输入: "42"
输出: 42

示例 2:
输入: "   -42"
输出: -42
解释: 第一个非空白字符为 &#39-&#39, 它是一个负号。
    我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。

示例3:
输入: "4193 with words"
输出: 4193
解释: 转换截止于数字 &#393&#39 ，因为它的下一个字符不为数字。

示例4:
输入: "words and 987"
输出: 0
解释: 第一个非空字符是 &#39w&#39, 但它不是数字或正、负号。
     因此无法执行有效的转换。
示例5:
输入: "-91283472332"
输出: -2147483648
解释: 数字 "-91283472332" 超过 32 位有符号整数范围。 
    因此返回 INT_MIN (&minus231) 。
```

## 解答
虽然实现起来不难，但是各种奇怪的情况需要考虑，所有很复杂。


```C++
class Solution {
public:
    int myAtoi(string str) {
        bool positive = true;
        str.erase(0, str.find_first_not_of(" "));
        if (str.front() == '-') {
            positive = false;
            str.erase(str.begin());
        } else if (str.front() == '+') {
            str.erase(str.begin());
        } else if (!isdigit(str.front())) {
            return 0;
        }

        str.erase(0, str.find_first_not_of("0"));

        int last_pos = str.find_first_not_of("1234567890");
        if (last_pos == string::npos) {
            last_pos = str.size();
        }
        string num = str.substr(0, last_pos);

        if (num.empty()) {
            return 0;
        }

        string int_max = to_string(INT_MAX);
        string int_min = int_max;
        int_min[int_min.size()-1] = '8';

        if ((num.size() > int_max.size()) || (num.size() == int_max.size()) && ((positive && num > int_max) || (!positive && num > int_min))) {
            return positive ? INT_MAX : INT_MIN;
        }
        
        int result = positive ? (num.front() - '0') : -(num.front() - '0');
        for (auto c : num.substr(1)) {
            result *= 10;
            result += positive ? (c - '0') : ('0' - c);
        }
        return result;
    }
};
```
