# Karger's Algorithm Implementation
This repository contains an implementation of Karger's algorithm, a randomized algorithm for computing minimum cuts in connected graphs. The algorithm was first introduced by David Karger in 1993.


## Performance Results

Peformance on graph instances:
| Graph File | (Vertices,Edges) | Min Cut | Avg Repetitions | Optimal Time | Average Time |
|------------|------------------|---------|-----------------|--------------|--------------|
|./grafi/g01.graph|(200,6022)|20|6.14|0.012s|0.142s|
|./grafi/g02.graph|(200,6955)|30|11.85|0.009s|0.231s|
|./grafi/g03.graph|(1000,124777)|30|2.44|0.211s|1.256s|
|./grafi/g04.graph|(1400,342305)|17|1.28|0.713s|2.322s|
|./grafi/g05.graph|(1400,342204)|42|1.92|0.698s|3.332s|
|./grafi/g06.graph|(2000,699036)|77|2.55|2.131s|14.463s|
|./grafi/g07.graph|(4000,360300)|77|41.00|0.807s|94.718s|
|./grafi/g08.graph|(4000,320205)|50|15.63|0.960s|33.946s|
|./grafi/g09.graph|(5000,625780)|99|31.75|2.219s|152.073s|
|./grafi/g10.graph|(5200,67279)|8|28.52|0.154s|10.973s|
|./grafi/g11.graph|(5200,67733)|10|60.01|0.156s|23.134s|
|./grafi/g12.graph|(6000,89726)|11|48.84|0.234s|25.034s|
|./grafi/g13.graph|(6200,96068)|12|28.01|0.235s|15.740s|
### Note

- **Min Cut**: The minimum cut value found by the algorithm (minimum number of edges that need to be removed to disconnect the graph into two components)
- **Avg Repetitions**: Average number of algorithm repetitions required to find the optimal solution (based on 500 test runs)
- **Optimal Time**: Time required to run the algorithm 200 times to reliably find the optimal solution
- **Average Time**: Total time for 500 runs of the repetition counting experiment



## Reference

Karger, David (1993). "Global Min-cuts in RNC and Other Ramifications of a Simple Mincut Algorithm". *Proceedings of the 4th Annual ACM-SIAM Symposium on Discrete Algorithms*.

## Usage
run cargo run --release