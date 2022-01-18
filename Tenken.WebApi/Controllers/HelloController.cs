using Microsoft.AspNetCore.Mvc;

namespace Tenken.WebApi.Controllers;

[ApiController]
[Route("[controller]")]
public class HelloController : ControllerBase
{
    private static readonly string Hello = "hello";

    private readonly ILogger<HelloController> _logger;

    public HelloController(ILogger<HelloController> logger)
    {
        _logger = logger;
    }

    [HttpGet(Name = "")]
    public String Get() => Hello;
}
