#include <lithium_http_server.hh>
            
int main() {
  li::http_api my_api;
              
  my_api.get("/") = 
  [&](li::http_request& request, li::http_response& response) {
    response.write("hello");
  };
  li::http_serve(my_api, 8080);
}
