function get_cristal () {
	$.ajax({
		url: 'http://163.5.245.219:3000/api/1/crystalmines',
		type: 'GET',
		xhrFields: { withCredentials: true },
		data: JSON.stringify({
			"username": localStorage.getItem("login"),
			"password": localStorage.getItem("pass")
		}),
	})
	.done(function(data) {
		$('#show_ressource > ul > li:nth-child(2) > span').empty().text(data.amount);
	})
	.fail(function() {
		console.log("error");
	});
}

function crystal()
{
	var d = new Date().getTime();
	$.ajax({
		url: 'http://163.5.245.219:3000/api/1/crystalmines',
		type: 'GET',
		xhrFields: { withCredentials: true },
		data: JSON.stringify({
			"username": localStorage.getItem("login"),
			"password": localStorage.getItem("pass")
		})
	})
	.done(function(data) {
		if (data.levelUpFinish > d) {
			$('#info_cristal .dev').text("Developpement en cours");
			setTimeout(usine(), data.levelUpFinish - d);
		}
		else
			$('#info_usine .dev').text("");
		$('#info_cristal .prod').text(data.production);
		$('#info_cristal .level').text(data.level);
		$('#info_cristal .metal').text(data.costNext.metal);
		$('#info_cristal .crystal').text(data.costNext.crystal);
	})
	.fail(function() {
		console.log("error");
	});

}

$(document).ready(function() {
	$('#dev_cristal').click(function() {
		$.ajax({
			url: 'http://163.5.245.219:3000/api/1/crystalmines/levelUp',
			type: 'PUT',
			xhrFields: { withCredentials: true },
			data: JSON.stringify({
				"username": localStorage.getItem("login"),
				"password": localStorage.getItem("pass")
			}),
		})
		.done(function() {
			crystal();
		})
		.fail(function(data) {
			console.log($.parseJSON(data.responseText).err);
		});
	});
});
