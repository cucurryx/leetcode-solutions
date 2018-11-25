# 470. 用 Rand7() 实现 Rand10()

## 问题
已有方法 `rand7` 可生成 1 到 7 范围内的均匀随机整数，试写一个方法 `rand10` 生成 1 到 10 范围内的均匀随机整数。

不要使用系统的 `Math.random()`

```
示例 1:

输入: 1
输出: [7]
示例 2:

输入: 2
输出: [8,4]
示例 3:

输入: 3
输出: [8,1,10]
 ```

提示:
- rand7 已定义。
- 传入参数: n 表示 rand10 的调用次数。
 

进阶:
- rand7()调用次数的 期望值 是多少 ?
- 你能否尽量少调用 rand7() ?

## 解答
先用一个rand7()随机数判断应该输出1-5还是6-10，然后再来一个rand7()来得到一个1-5的数，最后组合输出。

```C++
// The rand7() API is already defined for you.
// int rand7();
// @return a random integer in the range 1 to 7

class Solution {
public:
    int rand10() {
        int n;
        while ((n = rand7()) == 4);
        
        int m;
        while (true) {
            m = rand7();
            if (m == 6 || m == 7) {
                continue;
            }
            break;
        }
        
        return (n < 4 ? 0 : 5) + m;
    }
};
```