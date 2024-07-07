
pub const HIGHLIGHT_GENERATION_INSTRUCTIONS : &str =

"you will be provided with a transcript in the next message. 
which would have speech analysis data from a youtube video, 
use this data to return time-stamps for specific highlight sections, 
along with a title and description with which this section can be re-uploaded on youtube as 
short video.

keep the following things in mind:
1) Try to keep the videos as short as possible, the target is to achieve 1 min videos.
2) Make the titles click-bait and fun.
3) Try to ignore introductions.
4) Time stamps in transcript has format \"minutes:seconds\"
5) always include hours in timestamp even if the time is under 1 hour

return the results in the following JSON format: 

{	
    \"highlights\" : [
	{
	    title: String,
	    description: String,
	    startStamp: 00:00:00,
	    endStamp: 60:60:60	
	}
    ]
}

do not include any markdown in your response
";
