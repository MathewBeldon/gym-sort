# Gym Sort Algorithm

## Introduction
gym-sort™ is a solution to a problem that has baffled me: Why can't anyone put these weights back where they belong?

The primary goal of gym-sort™ is to maintain order in the weight rack area of a gym. It ensures that weights are easily findable and accessible, reducing the time spent searching for the right weight.

## How It Works
 1. When a gym user finishes using a weight, they approach the weight rack.
 2. The user identifies the correct spot for the weight.
 3. If the spot is empty, the user places the weight there.
 4. If the spot is occupied by an incorrectly placed weight, the user removes the misplaced weight and replaces it with the correct one.

To test that this would work in the real world we'll need to run a simulation...

## The Simulation
We created a state-of-the-art simulation, it's written in Rust, to prove that this algorithm is viable. With only 40% of people who are enlightened by the gym-sort™ algorithm, we can realistically self-sort the weight rack.

But wait, there's more! To make our simulation more simulationy we, in a stroke of optimistic genius, reckon that 10% of gym-goers will pick up stray weights, the true heroes of the gym floor.

And what's a good simulation that does not simulate the bad in people, we have the 20% - the non-believers, the anarchists of the gym-sort™ regime. These rebel scum deposit weights in any open spot they find, completely ignoring the prophecy of gym-sort™.

Below is the result of running tens of thousands of simulations, each involving up to 10 gym users performing a random number of sets ranging from 1 to 10. The key metric, and the one we've been pursuing all along, is `sets` - the average number of finished sets required to undo the damage caused by those unaware of their actions and restore the weight rack to its rightful state.

| gym-sort™ followers |  sets  |
|---------------------|--------|
| 10%                 |   3995 |
| 20%                 |   1563 |
| 30%                 |    825 |
| 40%                 |    515 |
| 50%                 |    330 |
| 60%                 |    230 |
| 70%                 |    186 |
| 80%                 |    149 |
| 90%                 |    125 |
| 100%                |    105 |