use super::super::database::Query::Query;

struct GetFileList {
    status: String,
    html: String,
    QueryPath: String,
    serialVersionUID: i64,
}

impl GetFileList {

    pub fn init (&mut self){
        self.serialVersionUID = 1;
    }

    pub fn new () -> GetFileList {
        GetFileList {
            status: String::new(),
            html: String::new(),
            QueryPath: String::new(),
            serialVersionUID: 1,
        }
    }

    pub fn setStatus (&mut self, nstatus: String){
        self.status = nstatus;
    }

    pub fn getStatus (&self) -> String {
        self.status.clone()
    }

    pub fn setQueryPath (&mut self, path: String) {
        self.QueryPath = path;
    }

    pub fn getQueryPath (&self) -> String {
        self.QueryPath.clone()
    }

    pub fn setHtml (&mut self, nhtml: String) {
        self.html = nhtml;
    }

    pub fn getHtml (&self) -> String {
        self.html.clone()
    }

    pub fn execute(&mut self) -> String {
        let query = Query::new();
        let tpath: Option<String> = Some(self.QueryPath);
        let file_array = query.queryFile_Bypath(tpath);

        self.html = self.html +
		"<tr class=\"file_list_back\">"+
			"<td> </td>"+
			"<td> <label><input type=\"checkbox\">&emsp;&emsp;</label><span class=\"glyphicon glyphicon-folder-open\"></span>&emsp;../</td>"+
			"<td></td>"+
			"<td></td>"+
        "</tr>";
        
        let mut return_val = String::new();

        if file_array.len() == 0 {
            self.status = String::from("false");
            return_val = String::from("success");
            return return_val;
        }
        else {
            self.status = String::from("true");
        }

        for i in 0..file_array.len() {
            self.html = self.html + 
			"<tr class=\"file_list_go\">"+
                "<td> </td>";
            if fileArray[i].is_folder() {
                self.html = self.html + "<td> <label><input type=\"checkbox\"></label> 　　<span class=\"glyphicon glyphicon-folder-open\"></span>　"
                + fileArray[i].get_name()+"</td>";
            }
            else {
                self.html = self.html 
                + "<td> <label><input type=\"checkbox\"></label> 　　<span class=\"glyphicon glyphicon-file\"></span>　"
                + fileArray[i].get_name()+"</td>"
            }
            self.html = self.html +    
				"<td>"+fileArray[i].get_attribute()+"</td>"+
				"<td>"+fileArray[i].get_time()+"</td>"+
			"</tr>";
        }

        return self.html.clone()
    }
}