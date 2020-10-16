# Color application
This applications displays a blank webpage with a color retrieved from the backend. It can also display the number of times a certain color is shown. This application is designed to showcase two different deployment strategies: blue-green deployments and canary deployments. The app is written in Rustlang because the author wanted to take a look at it.

## Deployment
The manifests folder contains two k8s deployments, one for the frontend and one for the backend. It will deploy the frontend and backend app as deployments along with its associated services. In addition an ingress yaml is provided for nginx ingress controller, that has to be fitted according to the individual k8s cluster.
After deploying the application you should see a blue website with a button that links to small chart that indicates the ratio between website with blue and green backgrounds.