# Task 1

## Prepare VM
Basic instructions: https://tsi-ccdoc.readthedocs.io/en/master/ResOps/2019/Minikube-and-NGINX-Practical-2019.html#exercise-0-0-starting-minikube

1. Follow instructions 0: Install Minikube and addons
2. ```[Optional]``` Install VNC, not required if you want to use port-forwarding

## Port forwarding
### Forwards
- Colorapp: ```ssh <VM-Host> -C -L 7000:colorapp:80```
- Linkerd: ```ssh <VM-Host> -C -L 50750:127.0.0.1:50750```
### Hosts
- Local machine: add ingress hostname to localhost alias ```/etc/hosts```  
- VM: add colorapp to ingress ip in ```/etc/hosts```


## Task description
0. Solutions can be found in the manifests/ folder try not to use them
1. Deploy the application described in this repo:
    - Backend image: quay.io/mariusdieckmann/colorapp-backend
    - Frontend image: quay.io/mariusdieckmann/colorapp-frontend
3. Create manifests to deploy the canary demo
    1. Secret
        - Needs to be mounted as ```APIKey``` env var
    2. Config => backend/config/config.yaml
        - Mount to /config/config.yaml
    3. Deployment frontend/backend + services; frontend only single replica
    4. Ingress frontend
    5. Add ingress IP with hostname for frontend in ```/etc/hosts```
4. Add well known labels to Deployment
5. Test it
6. Add appropriate requests and limits
    1. Test behaviour
7. Add HPA to backend with 30% CPU usage target
8. Create load and observe scaling
9. Configure nginx ingress to use sticky session and test it with the frontend