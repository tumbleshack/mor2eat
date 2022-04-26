# mor2eat

mor2eat is an application of Yan's algorithm to find the maximum ordered list of nodes visitable under a distance constraint. Specifically, it solves this graph problem:

Given a fully connected, directional graph G = (V, E) where each edge e_i has a weight w_i, find the path P = (v_1, v_2, … v_k) which maximizes the number of visited nodes k, but has total weight less than constraint d.

# Why?

I want to visit more Chick Fil A's than anyone ever has before in a single day. To do this, I need an optmial route (most CFA's, one day). Would it be easier to manually look at Google maps and construct one?

Probably.

But that's not as fun. Let's find the absolute optimal route 😎

# General Intuition

Make a graph, where Chick Fil A's are nodes, and directed, weighted edges between them represent the Google Maps estimated travel time. By inspection, the solution path will not have any leg longer than an hour, so the graph is not fully connected. Rather, only CFA's within about 35 miles of each other are connected.

Once the graph is built, repeatedly run Dijkstra's algorithm between a source and sink node to find the n shortest paths (in order). This process is known as Yan's algorithm. Of all paths under a certaint time, say 15.5 hours, find the one with the most nodes.

Run Yan's algorithm for all possible pairs of source/sink nodes, and again find the path with maximum nodes. Let's take that one.

# Time

Yan's runs in polynomial time, and we'll run it N^2 time. So while this algorithm is still polynomial, it's gonna be pretty darn slow. But that's okay, there's no rush.

# Practical Concerns

Stopping at CFA's is not instantaneous. We'll estimate a 10 minute average time spent at each CFA. We'll encode this in edge weights, where the weight of edge `e = (u,v) = driving_minutes_from_u_to_v + 10`.
