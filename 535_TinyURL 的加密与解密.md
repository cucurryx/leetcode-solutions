# 535.TinyURL 的加密与解密

## 问题
TinyURL是一种URL简化服务， 比如：当你输入一个URL`https://leetcode.com/problems/design-tinyurl`时，它将返回一个简化的URL`http://tinyurl.com/4e9iAk`.

要求：设计一个 TinyURL 的加密`encode`和解密`decode`的方法。

你的加密和解密算法如何设计和运作是没有限制的，你只需要保证一个URL可以被加密成一个TinyURL，并且这个TinyURL可以用解密方法恢复成原本的URL。

## 解答
这道设计题，就是需要我们给url哈希一下，然后存起来，自己设计一套规则，这样可以能够将short了之后的url还原，这个可以用一个哈希表实现。实际上，用一个数组也可以，把longUrl对应的索引，编码到shortUrl就好了。

```C++
class Solution {
private:
    unordered_map<string, string> m;
    int count = 0;
    
public:

    // Encodes a URL to a shortened URL.
    string encode(string longUrl) {
        string short_url = "http://tinyurl.com/" + to_string(count);
        m[short_url] = longUrl;
        ++count;
        return short_url;
    }

    // Decodes a shortened URL to its original URL.
    string decode(string shortUrl) {
        return m[shortUrl];   
    }
};

// Your Solution object will be instantiated and called as such:
// Solution solution;
// solution.decode(solution.encode(url));
```
