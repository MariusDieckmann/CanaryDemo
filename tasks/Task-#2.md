# Task 1
## Introduction

## Task description
1. Install linkerd https://linkerd.io/ CLI and install it into the cluster.
    - https://linkerd.io/2/getting-started/
2. Checkout interface ```linkerd dashboard &```
    1. Use ssh to port-forward the website with port 50750
3. Install flagger for canary releases https://linkerd.io/2/tasks/canary-release/
4. Try updating from 2.0 -> 2.1 and observe the behaviour
5. Try updating from 2.1 -> 2.2 and observe the behaviour
6. Delete and redeploy
    - Use HPA along with Canary to automatically scale the canary deployment
    - Scaling takes time, to see the results set appropriate timings
7. ```[Optional]``` Extra tasks
    - Connect ingress to linkerd: https://linkerd.io/2/tasks/using-ingress/