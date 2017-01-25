$(document).ready(function() {

	var refresh;

	console.log(refresh);
	function capitalize (str) {
		return str.charAt(0).toUpperCase() + str.slice(1);
	}

	function login()
	{
		$('title').text('Space Game -- Connexion');
		$('nav').hide();
		$('section').hide();
		$('aside').hide();
		$('#logout').hide();
		$('#connexion').show();
	}

	function maj_resource () {
		minion();
	}

	function game()
	{
		$('title').text('Space Game -- Accueil');
		recup_info();
		$('#connexion').hide();
		$('nav').show();
		$('section').show();
		$('aside').show();
		$('#logout').show();
		$('#ressource').hide();
		$('#usine').hide();
		$('#bataille').hide();
		$('#info_metal').hide();
		$('#info_cristal').hide();
		$('#info_usine').hide();
		$('#info_minion').hide();
		$('#showInscript').hide();
		if (refresh == null || refresh == undefined)
			maj_resource();
	}


	if (localStorage.getItem("login") == null)
		login();
	else
		game();

	$("#submit").click(function() {
		$.ajax({
			url: 'http://localhost:3000/api/1/players/register',
			type: 'POST',
			contentType: 'application/json',
			data: JSON.stringify({
				"email": $('#login').val(),
				"password": $('#password').val(),
				"username": $('#username').val()
			})
		})
		.done(function(data) {
			localStorage.setItem("login", $('#login').val());
			localStorage.setItem("userId", data.id);
			game();
		})
		.fail(function() {

		});
	});

	$('#reset').click(function() {
		$('#login').val("");
		$('#password').val("");
	});

	$('#logout').click(function() {
			localStorage.removeItem("login");
			localStorage.removeItem("userId");
			login();
	});

	$('nav > button').click(function() {
		var get_id;
		get_id = $(this).attr('id').replace("nav_", "");
		$('#info > .active').hide().attr('class', '');
		$("#" + get_id).show().attr('class', 'active');
		$('title').text('Space Game -- ' + capitalize(get_id));
		$('#show_info > .active_info').hide().attr('class', '');
		$('#show_info_usine > .active_info').hide().attr('class', '');
		if (get_id == "bataille") {
			get_info_minion();
		};

	    if (get_id == "usine") {
		$('#factory_number').show();
		$('#factories_display').show();
	    } else {
		$('#factory_number').hide();
		$('#factories_display').hide();
	    }
	});

	$('#box_img > .box_img_mine > img').click(function() {
		var get_info;
		get_info = $(this).attr('alt').replace("Mine de ", "info_");
		if ($('#show_info > .active_info').attr('id') == get_info)
			$('#show_info > .active_info').hide().attr('class', '');
		else {
			$('#show_info > .active_info').hide().attr('class', '');
			$('#' + get_info).show().attr('class', 'active_info');
		}
		get_info = get_info.replace("info_", "");
		if (get_info == "metal")
			metal();
		else if (get_info == "cristal")
			crystal();
	});

	$('#box_img_usine > .box_img_mine > img').click(function() {
		var get_info;
		get_info = "info_" + $(this).attr('alt');
		if ($('#show_info_usine > .active_info').attr('id') == get_info)
			$('#show_info_usine > .active_info').hide().attr('class', '');
		else {
			$('#show_info_usine > .active_info').hide().attr('class', '');
			$('#' + get_info).show().attr('class', 'active_info');
		}
		get_info = get_info.replace("info_", "");
		if (get_info == "usine")
			usine();
		else if (get_info == "minion")
			minion();
	});
});
