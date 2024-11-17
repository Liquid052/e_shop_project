
# VIS Eshop project

## Diesel migrations

Diesel is now embedded with sqlite into the app, meaning it shall be easier to test in cross-platform
environments like docker

### notes

- diesel now uses as its backend sqlite. In older version of the project, it used to be postgres
- if you plan to use WindowPlugin, remember that it takes control flow over the app (since it uses eframe). Otherwise,
  it runs in headless mode