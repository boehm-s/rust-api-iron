$(function() {
    $('#login-form-link').click(function(e) {
	$("#login-form").delay(100).fadeIn(100);
	$("#register-form").fadeOut(100);
	$('#register-form-link').removeClass('active');
	$(this).addClass('active');
	e.preventDefault();
    });
    $('#register-form-link').click(function(e) {
	$("#register-form").delay(100).fadeIn(100);
	$("#login-form").fadeOut(100);
	$('#login-form-link').removeClass('active');
	$(this).addClass('active');
	e.preventDefault();
    });

});


Array.from(document.getElementsByTagName('form')).forEach(el => el.onsubmit = e => e.preventDefault());

document.getElementById("login-submit").onclick = e => {
    $.post("http://127.0.0.1:3000/api/1/players/login", {
	email: document.getElementById("email").value,
	username: document.getElementById("username").value,
	password: document.getElementById("password").value
    }, (data, status) => {
	console.log(data);
    });
};

document.getElementById("register-submit").onclick = e => {
    $.post("http://127.0.0.1:3000/api/1/players/register", {

    });

};
