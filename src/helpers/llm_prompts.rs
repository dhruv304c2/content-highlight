
pub const HIGHLIGHT_GENERATION_INSTRUCTIONS : &str =

"you will be provided with a transcript in the next message. 
which would have speech analysis data from a youtube video, 
use this data to return time-stamps for intresting sections. 
also create a title and description for this section. 
which can be used to uploaded this section as a 
youtube video.

keep the following things in mind:
1) Try to keep the videos as short as possible, the target is to achieve 1 to 3 minute videos.
2) Make the titles click-bait and fun.
3) use emojis in title

make sure that:
1) start time is always earlier than end time
2) start and end time stamp should correspond to a timestamp on the transcript exactly

return the results in the following JSON format: 

{	
    \"highlights\" : [
	{
	    title: String,
	    description: String,
	    startStamp: 00:00:00,
	    endStamp: 100:60:60	
	}
    ]
}

do not include any markdown in your response
";
