# 609.Find Duplicate File in System

## 问题
Given a list of directory info including directory path, and all the files with contents in this directory, you need to find out all the groups of duplicate files in the file system in terms of their paths.

A group of duplicate files consists of at least two files that have exactly the same content.

A single directory info string in the input list has the following format: 
`"root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"`

It means there are n files (f1.txt, f2.txt ... fn.txt with content f1_content, f2_content ... fn_content, respectively) in directory root/d1/d2/.../dm. Note that n >= 1 and m >= 0. If m = 0, it means the directory is just the root directory.

The output is a list of group of duplicate file paths. For each group, it contains all the file paths of the files that have the same content. A file path is a string that has the following format: 

`"directory_path/file_name.txt"`


Example 1:
```
Input:
["root/a 1.txt(abcd) 2.txt(efgh)", "root/c 3.txt(abcd)", "root/c/d 4.txt(efgh)", "root 4.txt(efgh)"]
Output:  
[["root/a/2.txt","root/c/d/4.txt","root/4.txt"],["root/a/1.txt","root/c/3.txt"]]
```

Note:
- No order is required for the final output.
- You may assume the directory name, file name and file content only has letters and digits, and the length of file content is in the range of [1,50].
- The number of files given is in the range of [1,20000].
- You may assume no files or directories share the same name in the same directory.
- You may assume each given directory info represents a unique directory. Directory path and file info are separated by a single blank space.


Follow-up beyond contest: 

- Imagine you are given a real file system, how will you search files? DFS or BFS?
- If the file content is very large (GB level), how will you modify your solution?
- If you can only read the file by 1kb each time, how will you modify your solution?
- What is the time complexity of your modified solution? What is the most time-consuming part and memory consuming part of it? How to optimize?
- How to make sure the duplicated files you find are not false positive?

## 解答
基本的很容易，只需要简单处理下输入，然后建立一个content和path的`dict`就好了。

比较有意思的是给的思考题：
- 第一个是考虑如何在文件系统中搜索文件，一般都是用DFS。
- 第二个是如果文件特别大，达到GB级别如果做。应该是先把数据hash一下，可以根据文件大小，以及一些具有特征的数据进行hash来优化时间，实在不行，也可以按字节来进行比较。文件太大，只是需要自己来写一个hash算法。
- 第三个问题是如果内存很小，只能读入1kb数据，那就慢慢读吧，每次读一点就好。
- 最后分析时间复杂度，应该是每个文件都需要hash，而且每个文件都需要逐个字节的进行读取，所以应该是`O(m*n)`，m为文件数，n为文件长度。然后，建立了hash表之后，就很容易了。所以，整个程序最耗时间就是那个hash，同样也是耗内存的。


```python
class Solution:
    def findDuplicate(self, paths):
        d = {}
        for x in paths:
            tokens = x.split()
            root = tokens[0]
            for file in tokens[1:]:
                filename, content = file.split('(')[0], file.split('(')[-1][:-1]
                if content not in d:
                    d[content] = ['{}/{}'.format(root, filename)]
                else:
                    d[content].append('{}/{}'.format(root, filename))
        return [d[x] for x in d if len(d[x]) > 1]
```
