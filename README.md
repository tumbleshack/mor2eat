# mor2eat

mor2eat is an application of Yan's algorithm to find the maximum ordered list of Chick Fil A's visitable under a time constraint. 

# Why?

I want to visit more Chick Fil A's than anyone ever has before. To do this, I need an optmial route. Would it be easier to just make one myself?

Probably.

But that's not as fun. Let's find the absolute, mathematically most optimal route!

# General Intuition

Make a graph, where Chick Fil A's are nodes and directed, weighted edges between them represent the Google Maps estimated travel time. For practical concerns, the graph is not fully connected. 
Rather, CFA's are connected to any other CFA within about 35 miles. 

Once the graph is built, repeatedly Dijkstra's algorithm between a source and sink node to find the n shortest paths (in order). This process is known as Yan's algorithm.
Of all paths under a certaint time, say 15.5 hours, find the path with the most nodes.

Run Yan's algorithm for all pairs of source/sink nodes, and then find the path with maximum nodes. Let's take that one.
