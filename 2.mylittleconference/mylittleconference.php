<?php

// Load the input File
$workinglist = getInput('input.txt');
// Init the conference Array
$myconference = [];

// Calculate how many tracks are needed
$totaltime = 0;
foreach ($workinglist as $key => $value) {
	$totaltime = $totaltime + $value['time'];
}
// Round up and hope everything fits in
$idealMaxDays = ceil(($totaltime / (180 + 240))); 

// Init the tracks as long as there are tracks needed
for ($i=0; $i<$idealMaxDays; $i++) {
	// the track init for am and pm (can be written better)
	$myconference = setupTracks('am',$i, $myconference);
	$myconference = setupTracks('pm',$i, $myconference);

	// Fill the conference slots from the input file in the tracks
	foreach ($workinglist as $key => &$value) {
		// If the am Track as time left and if the talk isn't already booked add the talk
		if ($myconference[$i]['am']['time_left'] >= 10 && $workinglist[$key]['booked'] == '0') {
			$newconference = addTalktoTrack($myconference[$i]['am'], $value);
			// Update the conference with the talk
			$myconference[$i]['am'] = $newconference[0];
			// Update the talk status
			$workinglist[$key] = $newconference[1];
		}
		// If the pm Track as time left and if the talk isn't already booked add the talk
		if ($myconference[$i]['pm']['time_left'] >= 10 && $workinglist[$key]['booked'] == '0') {
			$newconference = addTalktoTrack($myconference[$i]['pm'], $value);
			// Update the conference with the talk
			$myconference[$i]['pm'] = $newconference[0];
			// Update the talk status
			$workinglist[$key] = $newconference[1];
		}
	}

}

// Create the output of the conference
printConference($myconference);

// create a visible representation of the result
function printConference($timetable) {

	// pack everthing in the <pre> tag that \n is honered
	echo "<pre>###‌‌ Test‌‌ output:‌\n";
	// Parse the timetable per track
	foreach ($timetable as $key => $value) {
		$keyo = $key + 1;
		echo "> Track {$keyo}:\n";
		// parse the timetable per talk entry
		foreach ($value as $key2 => $val2) {
			$firstrun = 0;
			// for as long as there are entries
			for ($i=0; $i<(count($val2)-3); $i++) {
				// the starttime depend on am (9x60=) or pm (1x60=)
				if ($key2 == 'am') {$starttime = '540';}
				if ($key2 == 'pm') {$starttime = '60';}
				// set the stark time only once per am / pm track
				if ($firstrun == 0) {$timetabletime = $starttime; $firstrun = 1;}

				// construct the time
				// the hour digits are rounded to full number and filled with 0 digits if no 2 digits 
				echo "> " . str_pad(round($timetabletime/60, 0, PHP_ROUND_HALF_DOWN), 2, "0", STR_PAD_LEFT)
					// minuit digits are modulo 60 filled with 0 digits if no 2 digits
					. ":" . str_pad($timetabletime%60, 2, "0")  
					// AM or PM
					. strtoupper($key2);

				// the subject and the time length of the talk
				echo " {$val2[$i]['subject']} {$val2[$i]['time']}min\n";

				// calculte the new starting time of the next talk
				$timetabletime = $timetabletime + $val2[$i]['time'];
			}
			// insert lunch after every AM
			// insert Networking Event after every PM
			if ($key2 == 'am') {echo "> 12:00PM Lunch\n";}
			if ($key2 == 'pm') {echo "> 05:00PM Networking Event\n"; echo "> \n";}
		}
	}

	echo "</pre>"; 
}

// Input the talk array and track array the insert
function addTalktoTrack($track, $talk) {

	// if there is enough time left
	if ($track['time_left'] > $talk['time']) {
		// insert the talk into the track
		// * note the track index is off by 3, because of meta infos of the track
		$track[count($track)-3] = array('subject' => $talk['subject'], 'time' => $talk['time']);
		// Calculte the time left of the track
		$track['time_left'] = $track['time_left'] - $talk['time'];
		// change the status of the talk to booked
		$talk['booked'] = '1';
	}
	// return updated track and talk
	return array($track,$talk);
}

// create an new am or pm track, if $array is empty a new one is created, else appended
function setupTracks($timeframe, $track, $array = []) {
	if ($timeframe == 'am') {
		$array[$track]['am'] = array('track' => 'am', 'time_total' => '180', 'time_left' => '180');
	} 
	if ($timeframe == 'pm') {
		$array[$track]['pm'] = array('track' => 'pm', 'time_total' => '240', 'time_left' => '240');
	} 
	return $array;
}

// read the input file and convert to array
function getInput($file) {
	$input = file_get_contents($file);
	// cut input into parts
	$input_list = explode("> ", $input);
	// create a new array
	$list = [];
	// parse input to fill in the list array
	foreach ($input_list as $key => $value)	{
		// incase of a lightning talk rename lightning to 5min
		if (strpos($value, 'lightning')) {
			$value = str_replace('lightning', '5min', $value);
		} 
		// if a part dont have a time (min) it isn't a part, skip it
		if (!strpos($value, 'min')) {continue;};
		
		// REGEX match every digit with one or many numbers follows by the letter min 
		preg_match("/\d{1,}min/", $value, $match1);
		// cut the min and save it as time in the list array
		$list[$key]['time'] = substr($match1[0],0,-3);

		// REGEX match non greedy till the first number in the string
		preg_match("/.+?(?=\d)/", $value, $match2);
		// save to subject in list array
		$list[$key]['subject'] = $match2[0];
		// status booked set to 0
		$list[$key]['booked'] = '0';
		
	}

	// sort the list array by talk length to get the long talks out of the way first
	usort($list, function ($a, $b) { return -($a['time'] <=> $b['time']); });
	return $list;
}

?>
