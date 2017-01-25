function minion () {
	var userId = localStorage.getItem("userId");
	$.ajax({
		url: 'http://localhost:3000/api/1/droids/get/' + userId,
		type: 'GET',
		xhrFields: { withCredentials: true },
	})
	.done(function(data) {
		var nbDroids = "NB Droids: " + data.length;
		document.getElementById('nbDroid').innerHTML = nbDroids;
	})
	.fail(function() {
		console.log("get droids error");
	});
}

$(document).ready(function() {
	$('#crea_minions').click(function() {
		var userId = localStorage.getItem("userId");
		$.ajax({
			url: 'http://localhost:3000/api/1/droids/create/'+ userId,
			type: 'POST',
			xhrFields: { withCredentials: true },
		})
		.done(function() {
			minion();
		})
		.fail(function(data) {
			console.log("create droids error");
		});
	});
});


