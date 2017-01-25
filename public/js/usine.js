function usine () {
	var d = new Date().getTime();
	$.ajax({
		url: 'http://163.5.245.219:3000/api/1/factorys',
		type: 'GET',
		xhrFields: { withCredentials: true },
		data: JSON.stringify({
			"username": localStorage.getItem("login"),
			"password": localStorage.getItem("pass")
		}),
	})
	.done(function(data) {
		if (data.levelUpFinish > d){
			$('#info_usine .dev').text("Developpement en cours");
			setTimeout(usine(), data.levelUpFinish - d);
		}
		else
			$('#info_usine .dev').text("");
		$('#info_usine .queue').text(data.queue.length);
		$('#info_usine .level').text(data.level);
		$('#info_usine .metal').text(data.costNext.metal);
		$('#info_usine .crystal').text(data.costNext.crystal);
	})
	.fail(function() {
		console.log("error");
	});
}

$(document).ready(function() {
	$('#dev_usine').click(function() {
		$.ajax({
			url: 'http://163.5.245.219:3000/api/1/factorys/levelUp',
			type: 'PUT',
			xhrFields: { withCredentials: true },
			data: JSON.stringify({
				"username": localStorage.getItem("login"),
				"password": localStorage.getItem("pass")
			}),
		})
		.done(function() {
			usine();
		})
		.fail(function(data) {
			console.log($.parseJSON(data.responseText).err);
		});
	});
});
