function get_metal () {
	$.ajax({
		url: 'http://163.5.245.219:3000/api/1/metalmines',
		type: 'GET',
		xhrFields: { withCredentials: true },
		data: JSON.stringify({
			"username": localStorage.getItem("login"),
			"password": localStorage.getItem("pass")
		})
	})
	.done(function(data) {
		$('#show_ressource > ul > li:nth-child(1) > span').empty().text(data.amount);
	})
	.fail(function() {
		console.log("error");
	});
}

function metal()
{
	var d = new Date().getTime();
	$.ajax({
		url: 'http://163.5.245.219:3000/api/1/metalmines',
		type: 'GET',
		xhrFields: { withCredentials: true },
		data: JSON.stringify({
			"username": localStorage.getItem("login"),
			"password": localStorage.getItem("pass")
		})
	})
	.done(function(data) {
		if (data.levelUpFinish > d) {
			$('#info_metal .dev').text("Developpement en cours");
			setTimeout(usine(), data.levelUpFinish - d);
		}
		else
			$('#info_usine .dev').text("");
		$('#info_metal .prod').text(data.production);
		$('#info_metal .level').text(data.level);
		$('#info_metal .metal').text(data.costNext.metal);
		$('#info_metal .crystal').text(data.costNext.crystal);
	})
	.fail(function() {
		console.log("error");
	});

}

$(document).ready(function() {
	$('#dev_metal').click(function() {
		$.ajax({
			url: 'http://163.5.245.219:3000/api/1/metalmines/levelUp',
			type: 'PUT',
			xhrFields: { withCredentials: true },
			data: JSON.stringify({
				"username": localStorage.getItem("login"),
				"password": localStorage.getItem("pass")
			}),
		})
		.done(function() {
			metal();
		})
		.fail(function(data) {
			console.log($.parseJSON(data.responseText).err);
		});
	});
});
