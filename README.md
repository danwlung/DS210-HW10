# DS210-HW10

This is my DS210 HW10 Fall 2023, 

32/40

− 4 pts
Grading comment:
There are no test cases

− 4 pts
Grading comment:
Detailed explanation of code missing

1. (30 points) In this question, the plan is to compute an approximation to PageRank.
PageRank was developed as a method for ranking web pages. Originally, the Google
search engine used PageRank.
PageRank is related to a memoryless process of walking over vertices of a directed
graph. We refer to this process as memoryless because the next vertex depends only on
the vertex at which we are currently and does not depend on previous vertices. How do
we then decide on our next vertex, i.e., how do we make one step in this walk? Suppose
that the current vertex is v. We select the next vertex as follows:
● If v has no outgoing edges, jump to a uniformly random vertex in the entire
graph.
● If v has at least one outgoing edge:
○ With probability 9/10, select uniformly at random one of them and follow
it.
○ Otherwise—i.e., with probability 1/10—jump to a vertex selected uniformly
at random from the entire graph.
Without going into details of how exactly PageRank is defined, we can compute its
approximation by simulating 100 random independent walks from each vertex. Each
random walk should consist of 100 random steps as described above. Our
approximation of PageRank for a vertex v is then the fraction of the random walks that
terminated at v (i.e., the number of random walks that terminated at v divided by 100n,
where n is the number of vertices). Notice that the sum of PageRank across all vertices
should add to 1.0 (modulo any precision errors in floating point arithmetic).
Input: Your program should read a directed graph from data.txt. The first line of the file
contains n, the number of vertices. Vertices are labeled 0 through n − 1. Each line of the
rest of the file describes one directed edge and consists of two vertex labels separated
by a space. The corresponding edge is a directed edge from the first vertex to the
second. Multiple edges between a pair of vertices are allowed on input. During the
neighbor selection process, each of them should be treated as a separate edge.
Output: Print out the labels of the five vertices with highest approximate values of
PageRank and their approximate PageRank values. (If the graph has less than five
vertices, output all of them.)
Sample input and output:
For the following data.txt:
5
0 3
1 3
2 3
3 4
4 3
4 0
your program might output:
vertex 3: approximate PageRank 0.426
vertex 4: approximate PageRank 0.324
vertex 0: approximate PageRank 0.214
vertex 2: approximate PageRank 0.024
vertex 1: approximate PageRank 0.012
For the following data.txt:
10
0 9
1 9
2 9
3 9
4 9
5 9
6 9
7 9
8 9
your program might output:
vertex 9: approximate PageRank 0.518
vertex 7: approximate PageRank 0.063
vertex 0: approximate PageRank 0.06
vertex 2: approximate PageRank 0.059
vertex 4: approximate PageRank 0.057
You can generate your own large file with at least 1000 vertices or use this file
https://github.com/kthanasi-git/ds210-demo/blob/main/pagerank_data.txt
which might produce the output
vertex 86: approximate PageRank 0.0019
vertex 566: approximate PageRank 0.0019
vertex 290: approximate PageRank 0.0019
vertex 282: approximate PageRank 0.0019
vertex 32: approximate PageRank 0.0018
Hint: By default, floating–point types do not come with an implementation of the Ord trait,
so they cannot be used for sorting or in a binary heap. To get around this, for this
problem, you can use the number of times random walks terminated in each of the
vertices to determine the top five. Once you know their labels, you can compute the
approximation to PageRank by dividing each of these numbers by 100n.
Hint2: Your output could be different due to the nature of random
2. Write at least one test verifying the functionality of your code.
3. (10 points) To receive the remaining 10 points your solution must use at least one
module that you placed in a separate file. The partition into modules should be
meaningful for the problem being solved.
