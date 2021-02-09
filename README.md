# Color application
This applications displays a webpage with a background color retrieved from the backend. It can also display the number of times a certain color is shown. This application is designed to showcase canary deployments and some other advanced Kubernetes Deployment techniques like heath checks and HPA. The app is written in Rustlang because the author wanted to take a look at it.

## Deployment
The deployment has two separate components, a frontend components and a backend component. The frontend has a simple website that displays a single color. The color is determined by a call to the backend that returns the displayed color in a short JSON message. Both the frontend and the backend listen by default on port 8000. The frontend can also the ratio of backend color results, which can be used to visualize deployment updates and the behavour of successding and failing canary deployments.

Both the frontend and the backend require the APIKey environment variable to be set in order to make calls. The backend additionally needs a configmap mounted into the following path: "/config/config.yaml". The config file can be found in the backend/config folder and can be used as a reference.

The frontend can also be used to create load on the backend for HPA demonstration.

The frontend can currently only be deployed with a single replica, due to problems with the load generator.

The backend has an additional health endpoint under /health that can be used for liveness and readiness probes. Whether the healthchecks fail or not can be determined by the confiig file.

## Version Branches
Each branch represents a separate version of the canary demo. The version 1.0 will display a simple website green website, v1.1 will cause a 500 range error and v1.2 will result in  a blue website. This is only used to demonstrate and test canary deployments.

## Variables
Frontend: 
    - ```APIKey```: Secret key that added for authentication, shared with the backend, should be the same secret
Backend: 
    - ```APIKey```: Secret key that added for authentication, shared with the frontend, should be the same secret

## APIs
Both APIs listen on port ```8000```. The default port can be changed with the ```ROCKET_PORT``` environment variable. 

### Frontend
The following endpoints are available:
- GET ```/```: base website
- GET ```/stats/data```: A statistical representation of the colors returned by the backend
- GET ```stats/show```: A visual representation of the stats from ```/stats/data```
- GET ```/load/start```: Starts creating load for the backend
- GET ```/load/stop```: Stops creating load for the backend


### Backend
The following endpoints are available:
- GET ```/health/live```: Checks if the application is health
- GET ```/health/startup```: Delays the application ready state

- GET ```/color```: Returns the defined color
- GET ```/load```: Can be used to generate load on the server to observe scaling