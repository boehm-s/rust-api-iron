$(document).ready(function() {
    $("#submit").click(function() {
        $.ajax({
            url: 'http://localhost:3000/api/1/players/register',
            type: 'POST',
            contentType: 'application/json',
            headers: {"Access-Control-Allow-Origin": "localhost:3000"},
            data: JSON.stringify({  
                "email": $('#login').val(),
                "password": $('#password').val(),
                "username": $('#username').val()
            }),
        })
        .done(function(data) {
            localStorage.setItem("login", $('#login').val());
            localStorage.setItem("pass", $('#password').val());
            game();
        })
        .fail(function() {

        });
    });
});