# cos-event
An IBM Cloud Object Store Event Processor

Deploy this application with Code Engine Build
https://cloud.ibm.com/docs/codeengine?topic=codeengine-build-image 

Create a Cloud Object Store bucket with an event subscription and attach it to the app.
https://cloud.ibm.com/docs/codeengine?topic=codeengine-cli#cli-subscription

View the logs 
```
$ ibmcloud ce app logs -n cos-event-app-e1
Project 'events-demo' and all its contents will be automatically deleted 7 days from now.
Getting application 'cos-event-app-e1'...
Getting revisions for application 'cos-event-app-e1'...
Getting instances for application 'cos-event-app-e1'...
Getting logs for all instances of application 'cos-event-app-e1'...
OK

cos-event-app-e1-98l7z-deployment-d8c5996db-9hgdj/user-container:    
{"level":30,"time":1607624067727,"msg":"Logger started","level":Info}  
      Running server on: http://localhost:8080/  
{"level":30,"time":1607624067728,"msg":"Server listening on http://0.0.0.0:8080"}  
{"level":30,"time":1607624069107,"msg":"<-- Request received","method":POST,"path":/}  
{"bucket":"cos-code-engine-demo","endpoint":"","key":"circdna.gb","notification":{"bucket_name":"cos-code-engine-demo","content_type":"application/x-gameboy-rom","event_type":"Object:Write","format":"2.0","object_etag":"db3c21eb1c5beb4be1975302bb2f3e3c","object_length":"16067","object_name":"circdna.gb","request_id":"165f2b57-f009-44d4-8f71-80dba52399fe","request_time":"2020-12-10T18:14:16.738Z"},"operation":"Object:Write"}  
{"level":30,"time":1607624069107,"msg":"--> Response sent","method":POST,"path":/,"status":200 - OK,"duration":88.777µs} 
```