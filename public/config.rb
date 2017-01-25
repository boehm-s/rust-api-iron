http_path = "./"
css_dir = "./css/"
sass_dir = "./scss/"
images_dir = "./img/"
javascript_dir = "./js/"
font_dir = "./font/"

environment = :development # :development OR :production
output_style = (environment == :production) ? :compressed : :expanded
line_comments = false #(environment == :production) ? false : true
