## Php setup 

```json	
{
    "test": "vendor/bin/phpunit",
    "fmt": "phpcs --standard=PSR12 app",
    "lint": "phpstan analyse app --level 9",
    "doc": "phpDocumentor -d app -t docs",
}
```

