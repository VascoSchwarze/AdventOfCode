import java.io.*;
import java.util.*;
import java.util.Arrays;

public class Solution {

	public static void main(String[] args) {
		Tag17();
	}

	public static void Tag4() {
		int passportIndex = 0;
		String passports[] = new String [500];
		
		String fields[] = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
		int fieldsLen = fields.length;
		
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input4.txt"));
			String line;
			String currentPassport = "";
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				//debug(line);
				if(line == null) {
					eof = true;
					line = "";
				}
				
				if(line.length() > 1) {
					currentPassport = currentPassport +" "+ line;
				}
				else {
					passports[passportIndex] = currentPassport;
					passportIndex++;
					currentPassport = "";
				}
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		
		int valid = 0;
		
		for(int i = 0; i < passportIndex; i++) {
			int contains = 0;
			for (int j = 0; j < fieldsLen; j++) {
				if(passports[i].contains(fields[j])) {
					contains++;
				}
			}
			
			if(contains == fieldsLen) {
				String valueOfFields[] = passports[i].split(" ");
				int validData = 0;
				for (int k = 0; k < valueOfFields.length; k++) {
					for (int l = 0; l < fieldsLen; l++) {
						if(valueOfFields[k].contains(fields[l])) {
							String data[] = valueOfFields[k].split(":");
							if(Tag4_dataIsValid(data[0], data[1]))
								validData++;
						}		
					}
				}
				System.out.println(validData);
				if(validData == fieldsLen)
					valid++;
			}
				
		}
		
		System.out.println(valid);
		
	}
	
	static boolean Tag4_dataIsValid (String field, String value) {
		switch (field) {
		case "byr": if(Integer.parseInt(value) < 1920 || Integer.parseInt(value) > 2002) return false;
					else return true;
		case "iyr": if(Integer.parseInt(value) < 2010 || Integer.parseInt(value) > 2020) return false;
					else return true;
		case "eyr": if(Integer.parseInt(value) < 2020 || Integer.parseInt(value) > 2030) return false;
					else return true;
		case "hgt": if(value.matches("\\b1([5-8][0-9]|9[0-3])cm|\\b(59|6[0-9]|7[0-6])in")) { debug("hgt match"); return true; } else return false;
		case "hcl": if (value.matches("#([0-9]|[a-f]){6}")) { debug("hcl match"); return true;} else return false;
		case "ecl": if(value.contains("amb") || value.contains("blu") || value.contains("brn") || value.contains("gry") || value.contains("grn") || value.contains("hzl") || value.contains("oth")) return true;
					else return false;
		case "pid": return value.matches("[0-9]{9}");
			
		}
		
		return false;
	}
	
	public static void Tag6() {
		int groupIndex = 0;
		String groups[] = new String [800];
		String groups2[][] = new String[800][11];
		int groups2AnsCount[] = new int[800];
		int currentGroupIndex = 0;
		
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input5.txt"));
			String line;
			String currentGroup = "";
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					line = "";
				}
				
				if(line.length() > 0) {
					currentGroup = currentGroup + line;
					//Teil 2
					groups2[groupIndex][currentGroupIndex] = line;
					currentGroupIndex++;
					
				}
				else {
					groups[groupIndex] = currentGroup;
					groups2AnsCount[groupIndex] = currentGroupIndex;
					groupIndex++;
					currentGroupIndex = 0;
					currentGroup = "";
				}
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		
		String groupsNoDupl[] = new String [800];
		for(int i = 0; i < groupsNoDupl.length; i++) {
			groupsNoDupl[i] = "";
		}
		
		String groups2Dupl[]= new String [800];
		for(int i = 0; i < groups2Dupl.length; i++) {
			groups2Dupl[i] = "";
		}
		
		for(int i = 0; i < groupIndex; i++) {
			for(int j = 0; j < groups[i].length(); j++) {
				if(!groupsNoDupl[i].contains(groups[i].substring(j, j+1))) {
					groupsNoDupl[i] = groupsNoDupl[i].concat(groups[i].substring(j,j+1));
					groups2Dupl[i] = groups2Dupl[i].concat(groups[i].substring(j,j+1));
				}
			}
			
			
			for(int k = 0; k < groupsNoDupl[i].length(); k++) {
				String currentString = groupsNoDupl[i].substring(k,k+1);
				
				for(int l = 0; l < groups2AnsCount[i]; l++) {
					String ans = groups2[i][l];
					if(!ans.contains(currentString)) {
						if(groups2Dupl[i].contains(currentString)) {
							int index = groups2Dupl[i].indexOf(currentString);
							groups2Dupl[i] = groups2Dupl[i].substring(0,index).concat(groups2Dupl[i].substring(index+1, groups2Dupl[i].length()));
							break;
						}
					}
				}
			}
			
		}
		
		int count = 0;
		for(int k = 0; k < groupsNoDupl.length; k++) {
			count += groupsNoDupl[k].length();
		}
		
		
		System.out.println(count);
		
		int count2 = 0;
		for (int m = 0; m < groups2Dupl.length; m++) {
			count2 += groups2Dupl[m].length();
		}
		
		System.out.println(count2);
		
	}
	
	public static void Tag7() {
		HashMap<String, Set<String>> map = new HashMap<String,Set<String>>();

		HashMap<String, Vector<String>> vectorMap = new HashMap<String, Vector<String>>();
		String wantedBag = "shiny gold";
		
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input7.txt"));
			String line;
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					break;
				}
				
				Set<String> content = new HashSet<String>();
				
				Vector<String> contentVector = new Vector<String>();
				
				String[] rule =  line.split(" bags contain ");
				if(!rule[1].contains("no")) {
					for(int i = 0; i < rule[1].split(",").length; i++) {
						String bagColor = rule[1].split(", ")[i].split(" ")[1] +" "+ rule[1].split(", ")[i].split(" ")[2];
						content.add(bagColor);
						for(int j = 0; j < Integer.parseInt(rule[1].split(", ")[i].split(" ")[0]); j++) {
							contentVector.add(bagColor);
						}
					}
				}
				
				map.put(rule[0], content);
				vectorMap.put(rule[0], contentVector);
				
			}
			
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		
		Set<String> possBags = new HashSet<String>();
		
		for(String key : map.keySet()) {
			if(map.get(key).contains(wantedBag)) {
				possBags.add(key);
			}
		}
		
		int newFound = 1;
		
		while(newFound != 0) {
			Set<String> possBagsCopy = new HashSet<String>(possBags);
			newFound = 0;
			for(String bag : possBagsCopy) {
				for(String key : map.keySet()) {
					if(map.get(key).contains(bag)) {
						if(possBags.add(key)) {
							newFound++;
						}
					}
				}
			}
			
		}
		
		//Teil 2
		
		Vector<String> neededBags = new Vector<String>();
		neededBags.addAll(vectorMap.get(wantedBag));
		
		int count = neededBags.size();
		
		for(String bag : neededBags) {
			count += Tag7_count(vectorMap, bag);
		}
	
		debug(count);
	}
	
	public static int Tag7_count(HashMap<String,Vector<String>> map, String bag) {
		int ret = map.get(bag).size();
		for(String bagContent : map.get(bag)) {
			ret += Tag7_count(map, bagContent);
		}
		return ret;
	}
	
	public static void Tag8() {
		BufferedReader reader;
		ArrayList<String> instructions = new ArrayList<String>();
		
		try {
			reader = new BufferedReader(new FileReader("input/input8.txt"));
			String line;
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					line = "";
					break;
				}
				
				instructions.add(line);
				
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		
		
		for(int i = 0; i < instructions.size(); i++) {
			if(instructions.get(i).startsWith("jmp")) {
				ArrayList<String> newInstructions = new ArrayList<String>(instructions);
				newInstructions.set(i, instructions.get(i).replace("jmp", "nop"));
				if(Tag8_tryChange(newInstructions)) {
					instructions.set(i, instructions.get(i).replace("jmp", "nop"));
					debug(i + " 0");
					break;
				}
			}
			else {
				if(instructions.get(i).startsWith("nop")) {
					ArrayList<String> newInstructions = new ArrayList<String>(instructions);
					newInstructions.set(i, instructions.get(i).replace("nop", "jmp"));
					if(Tag8_tryChange(newInstructions)) {
						instructions.set(i, instructions.get(i).replace("nop", "jmp"));
						debug(i + " 1");
						break;
					}
				}
			}
		}
		
		
		int index = 0;
		boolean executed[] = new boolean[instructions.size()+1];
		int accumulator = 0;
		while (!executed[index] && index < instructions.size()) {
			executed[index] = true;
			if(instructions.get(index).startsWith("nop")) {
				index++;
			}
			else {
				if(instructions.get(index).startsWith("acc")) {
					String instruc[] = instructions.get(index).split(" ");
					if(instruc[1].startsWith("+")) {
						accumulator += Integer.parseInt(instruc[1].substring(1, instruc[1].length()));
					}
					else {
						accumulator -= Integer.parseInt(instruc[1].substring(1, instruc[1].length()));
					}
					index++;
				}
				else {
					String instruc[] = instructions.get(index).split(" ");
					if(instruc[1].startsWith("+")) {
						index += Integer.parseInt(instruc[1].substring(1, instruc[1].length()));
					}
					else {
						index -= Integer.parseInt(instruc[1].substring(1, instruc[1].length()));
					}
				}
			}
		}
		
		
		debug(accumulator);
		
	}
	
	static boolean Tag8_tryChange(ArrayList<String> instructions) {
		int index = 0;
		boolean executed[] = new boolean[instructions.size()+1];
		int accumulator = 0;
		while (!executed[index] && index < instructions.size()) {
			executed[index] = true;
			if(instructions.get(index).startsWith("nop")) {
				index++;
			}
			else {
				if(instructions.get(index).startsWith("acc")) {
					String instruc[] = instructions.get(index).split(" ");
					if(instruc[1].startsWith("+")) {
						accumulator += Integer.parseInt(instruc[1].substring(1, instruc[1].length()));
					}
					else {
						accumulator -= Integer.parseInt(instruc[1].substring(1, instruc[1].length()));
					}
					index++;
				}
				else {
					String instruc[] = instructions.get(index).split(" ");
					if(instruc[1].startsWith("+")) {
						index += Integer.parseInt(instruc[1].substring(1, instruc[1].length()));
					}
					else {
						index -= Integer.parseInt(instruc[1].substring(1, instruc[1].length()));
					}
				}
			}
		}
		
		if(executed[index])
			return false;
		else
			return true;
	}
	
	public static void Tag9() {
		ArrayList<Long> list = new ArrayList<Long>();
		int preambleLen = 25;
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input9.txt"));
			String line;
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					line = "";
					break;
				}
				list.add(Long.parseLong(line));
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		
		long endNumber = 0;
		for(int i = preambleLen; i < list.size(); i++) {
			boolean found = false;
			for(int j = i-preambleLen; j < i-1; j++) {
				for(int k = j + 1; k < i; k++) {
					if(list.get(j)+list.get(k) == list.get(i)) {
						found = true;
						break;
					}
				}
			}
			
			if(!found) {
				endNumber = list.get(i);
				break;
			}
		}
		
		long largeNumber = 0;
		long smallNumber = 0;
		
		for(int i = 2; i < list.size(); i++) {
			for(int j = 0; j < list.size()-i; j++) {
				long sum = 0;
				long smallest = Long.MAX_VALUE;
				long largest = 0;
				for(int k = 0; k < i; k++) {
					long num = list.get(j+k);
					sum += num;
					if(num < smallest) {
						smallest = num;
					}
					if(num > largest) {
						largest = num;
					}
				}
				if(sum == endNumber) {
					largeNumber = largest;
					smallNumber = smallest;
					i = list.size();
					break;
				}
			}
		}
		
		
		debug(endNumber);
		
		debug(smallNumber);
		debug(largeNumber);
		debug(largeNumber+smallNumber);
	}
	
	public static void Tag10() {
		ArrayList<Integer> list = new ArrayList<Integer>();
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input10.txt"));
			String line;
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					line = "";
					break;
				}
				list.add(Integer.parseInt(line));
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		list.add(0);
		Collections.sort(list);
		list.add(list.get(list.size() - 1) + 3);
		
		int oneDiffCount = 0;
		int threeDiffCount = 0;
		
		for(int i = 0; i < list.size() - 1; i++) {
			if(list.get(i+1) - list.get(i) == 1) {
				oneDiffCount++;
			}
			else {
				if(list.get(i+1) - list.get(i) == 3) {
					threeDiffCount++;
				}
			}
		}

		//Teil 2

		double count = 1;
		debug(list);
		int chains[] = new int[5];
		int lastAd = 0;
		int currentChain = 0;
		for(int i = 1; i < list.size(); i++) {
			if(list.get(i) - lastAd == 1) {
				currentChain++;
			}
			else {
				chains[currentChain]++;
				currentChain = 0;
			}
			lastAd = list.get(i);
		}
		
		debug(chains[1]);
		debug(chains[2]);
		debug(chains[3]);
		debug(chains[4]);
		
		count = Math.pow(2, chains[2]) * Math.pow(4, chains[3]) * Math.pow(7,  chains[4]);
		System.out.printf("%f", count);
		debug(oneDiffCount);
		debug(threeDiffCount);
		debug(oneDiffCount * threeDiffCount);
	}

	public static void Tag11() {
		int height = 98, width = 98;
		
		char seats[][] = new char[height+2][width+2];
		int rowCount = 1;
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input11.txt"));
			String line;
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					line = "";
					break;
				}
				seats[rowCount][0] = '.';
				for(int i = 1; i < line.length()+1; i++) {
					seats[rowCount][i] = line.charAt(i-1);
				}
				seats[rowCount][width+1] = '.';
				rowCount++;
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}		
		
		HashMap<ArrayList<Integer>, Character> changes;
		boolean changed = true;
		while(changed) {
			changes = new HashMap<ArrayList<Integer>, Character>();
			changed = false;
			for(int i = 1; i < seats.length - 1; i++) {
				for(int j = 1; j < seats[i].length - 1; j++) {
					if(seats[i][j] == 'L') {/*
						if(seats[i-1][j] != '#' && seats[i-1][j+1] != '#' 
								&& seats[i-1][j-1] != '#' && seats[i][j-1] != '#'
								&& seats[i][j+1] != '#' && seats[i+1][j-1] != '#'
								&& seats[i+1][j] != '#' && seats[i+1][j+1] != '#') {
							*/
						//Teil 2
						if(!Tag11_CheckVision(i,j,seats,-1,-1) && !Tag11_CheckVision(i,j,seats,-1,0)
								&& !Tag11_CheckVision(i,j,seats,-1,1) && !Tag11_CheckVision(i,j,seats,0,-1)
								&& !Tag11_CheckVision(i,j,seats,0,1) && !Tag11_CheckVision(i,j,seats,1,-1)
								&& !Tag11_CheckVision(i,j,seats,1,0) && !Tag11_CheckVision(i,j,seats,1,1)) {
							ArrayList<Integer> coords = new ArrayList<Integer>();
							coords.add(i);
							coords.add(j);
							changes.put(coords, '#');
							changed = true;
						}
					}
					
					if(seats[i][j] == '#') {
						int occSeats = 0;
						for(int l = 0; l < 3; l++) {
							for(int m = 0; m < 3; m++) {
								if(l == 1 && m == 1) {}
								else {
									if(Tag11_CheckVision(i,j,seats, l-1, m-1)) {
										occSeats++;
									}
								}
							}
						}
						
						
						if(occSeats >= 5) {
							ArrayList<Integer> coords = new ArrayList<Integer>();
							coords.add(i);
							coords.add(j);
							changes.put(coords, 'L');
							changed = true;
						}
					}
				}
			}
			for(ArrayList<Integer> coords : changes.keySet()) {
				char newc = changes.get(coords);
				seats[coords.get(0)][coords.get(1)] = newc;
			}
		}

		int totalOcc = 0;
		for(int i = 1; i < seats.length-1; i++) {
			for(int j = 1; j < seats[i].length-1; j++) {
				if(seats[i][j] == '#') {
					totalOcc++;
				}
			}
		}
		debug(totalOcc);		
	}
	
	static boolean Tag11_CheckVision (int x, int y, char[][] seats, int xDir, int yDir) {
		boolean ret = false;
		char lookAt = '.';
		while(lookAt == '.') {
			x = x+xDir;
			y = y+yDir;
			if(x < 0 || x >= seats.length || y < 0 || y >= seats[0].length) 
				return false;
			lookAt = seats[x][y];
		}		
		if(lookAt == '#')
			ret = true;
		return ret;
	}
	
	static void Tag12() {
		ArrayList<String> nav = new ArrayList<String>();
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input12.txt"));
			String line;
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					line = "";
					break;
				}
				nav.add(line);
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		
		//Teil 1
		int nsPos = 0, ewPos = 0, degrees = 90;
		
		for(String ins : nav) {
			int value = Integer.parseInt(ins.substring(1));
			switch (ins.charAt(0)) {
			case 'N': nsPos += value; break;
			case 'S': nsPos -= value; break;
			case 'E': ewPos += value; break;
			case 'W': ewPos -= value; break;
			case 'L': degrees -= value; if(degrees < 0) degrees += 360; break;
			case 'R': degrees += value; if(degrees > 270) degrees -= 360; break;
			case 'F':	switch (degrees) {
						case 0: nsPos += value; break;
						case 90: ewPos += value; break;
						case 180: nsPos -= value; break;
						case 270: ewPos -= value; break;
						}
			}
		}
		
		//Teil 2
		int wpNSPos = 1, wpEWPos = 10, shipNSPos = 0, shipEWPos = 0;
		
		for(String ins : nav) {
			int value = Integer.parseInt(ins.substring(1));
			switch (ins.charAt(0)) {
			case 'N': wpNSPos += value; break;
			case 'S': wpNSPos -= value; break;
			case 'E': wpEWPos += value; break;
			case 'W': wpEWPos -= value; break;
			case 'F': shipNSPos += value * wpNSPos; shipEWPos += value * wpEWPos; break;
			case 'L': 	switch(value) {
						case 90: int ns = wpNSPos; wpNSPos = wpEWPos; wpEWPos = -ns; break;
						case 180: wpNSPos = -wpNSPos; wpEWPos = -wpEWPos; break;
						case 270: int ns1 = wpNSPos; wpNSPos = -wpEWPos; wpEWPos = ns1; break;
					  	}
						break;
			case 'R': 	switch(value) {
						case 90: int ns = wpNSPos; wpNSPos = -wpEWPos; wpEWPos = ns; break;
						case 180: wpNSPos = -wpNSPos; wpEWPos = -wpEWPos; break;
						case 270: int ns1 = wpNSPos; wpNSPos = wpEWPos; wpEWPos = -ns1; break;
		  				}
			}
		}
		
		debug("Teil 1");
		debug(Math.abs(nsPos)+Math.abs(ewPos));
		debug("Teil2");
		debug(Math.abs(shipNSPos)+Math.abs(shipEWPos));
		
	}
	
	static void Tag13() {
		int firstTimestamp = 0;
		ArrayList<Integer> busses = new ArrayList<>();
		ArrayList<Integer[]> departs = new ArrayList<>();
		int maxBus = 0, maxBusOffset = 0;
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input13.txt"));
			String line = reader.readLine();
			firstTimestamp = Integer.parseInt(line);
			line = reader.readLine();
			String lines[] = line.split(",");
			int offset = 0;
			for(String s : lines) {
				if(!s.contains("x")) {
					int busNum = Integer.parseInt(s);
					busses.add(busNum);
					Integer depart[] = {busNum, offset};
					departs.add(depart);
					if(busNum > maxBus) {
						maxBus = busNum;
						maxBusOffset = offset;
					}
				}
				offset++;
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		//Teil 1
		int waitTime = Integer.MAX_VALUE, firstBus = 0;
		for(int bus : busses) {
			int time = 0;
			while(time < firstTimestamp) {
				time += bus;
			}
			if(time - firstTimestamp < waitTime) {
				firstBus = bus;
				waitTime = time - firstTimestamp;
			}
		}
		//debug(waitTime);
		//debug(firstBus);
		debug(waitTime * firstBus);
		
		//Teil 2
		boolean found = false;
		long time = maxBus - maxBusOffset;
		long inc = maxBus;
		int maxCorrDeps = 1, newMCD = 1;
		long newMCDTime = 0;
		while (!found) {
			int correctDeparts = 0;
			for(Integer[] dep : departs) {
				if((time + dep[1]) % dep[0] == 0) {
					correctDeparts++;
				}
				else {
					break;
				}
			}
			if(correctDeparts == departs.size()) {
				found = true;
			}
			else {
				if(correctDeparts > maxCorrDeps) {
					if(newMCD == correctDeparts) {
						maxCorrDeps = newMCD;
						inc = time - newMCDTime;
					}
					else {
						newMCD = correctDeparts;
						newMCDTime = time;
					}
				}
				time += inc;
			}
			//debug(time);
		}
		debug(time);
	}
	
	static void Tag14() {
		ArrayList<String> instructions = new ArrayList<String>();
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/input14.txt"));
			String line;
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					break;
				}
				instructions.add(line);
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		
		HashMap<Long, Long> memory = new HashMap<>();
		String mask = "";
		for (String ins : instructions) {
			if(ins.contains("mask")) {
				mask = ins.split(" = ")[1];
			}
			else {
				long address = Long.parseLong(ins.split("\\[")[1].split("\\]")[0]);
				long value = Long.parseLong(ins.split(" = ")[1]);
				/* Teil 1 
				String valueBinary = String.format("%36s", Long.toBinaryString(value)).replace(" ", "0");
				for(int i = 0; i < 36; i++) {
					if(!(mask.charAt(i) == 'X')) {
						valueBinary = valueBinary.substring(0,i) + mask.charAt(i) + valueBinary.substring(i+1);
					}
				}
				value = Long.parseLong(valueBinary, 2);
				if(memory.containsKey(address)) {
					memory.replace(address, value);
				}
				else {
					memory.put(address,value);
				}
				*/
				//Teil 2
				String addressBinary = String.format("%36s", Long.toBinaryString(address)).replace(" ", "0");
				for(int i = 0; i < 36; i++) {
					if(!(mask.charAt(i) == '0')) {
						addressBinary = addressBinary.substring(0,i) + mask.charAt(i) + addressBinary.substring(i+1);
					}
				}
				ArrayList<String> addressesBinary = new ArrayList<>();
				addressesBinary.add(addressBinary);
				ArrayList<String> newAddrBinary;
				boolean changed;
				do {
					changed = false;
					newAddrBinary = new ArrayList<>();
					for(String addr : addressesBinary) {
						boolean changedThisAddr = false;
						for(int i = 0; i < 36; i++) {
							if(addr.charAt(i) == 'X') {
								newAddrBinary.add(addr.substring(0,i) + '0' + addr.substring(i+1));
								newAddrBinary.add(addr.substring(0,i) + '1' + addr.substring(i+1));
								changed = true;
								changedThisAddr = true;
								break;
							}
						}
						if(!changedThisAddr)
							newAddrBinary.add(addr);
					}
					addressesBinary = new ArrayList<>(newAddrBinary);
					
				} while (changed);
				
				for(String addrBin : addressesBinary) {
					long addr = Long.parseLong(addrBin, 2);
					if(memory.containsKey(addr)) {
						memory.replace(addr, value);
					}
					else {
						memory.put(addr,value);
					}
				}				
			}
		}
		long sum = 0;
		for(long key : memory.keySet()) {
			sum += memory.get(key);
		}
		debug(sum);
	}
	
	static void Tag15() {
		int numInput[] = {2,1,10,11,0,6};
		ArrayList<Integer> numbers = new ArrayList<>();
		HashMap<Integer, Integer> lastIndex = new HashMap<>();
		int index = 1;
		for(int num : numInput) {
			numbers.add(num);
			lastIndex.put(num, index);
			index++;
		}		
		
		while (numbers.size() <= 2020) {
			int lastNum = numbers.get(numbers.size() - 1);
			if(numbers.indexOf(lastNum) == numbers.size() - 1) {
				numbers.add(0);
			}
			else {
				numbers.remove(numbers.size() - 1);
				int lastIdx = numbers.lastIndexOf(lastNum);
				numbers.add(lastNum);
				numbers.add(numbers.size() - (lastIdx + 1));
			}
		}
		debug(numbers.get(2019));
		
		//Teil 2
		int nextNum = 0;
		while(index < 30000000) {
			int newNum = nextNum;
			if(lastIndex.containsKey(newNum)) {
				nextNum = index - lastIndex.get(newNum);
				lastIndex.replace(newNum, index);
			}
			else {
				nextNum = 0;
				lastIndex.put(newNum, index);
			}
			index++;
		}
		debug(nextNum);
	}
	
	static void Tag16() {
		ArrayList<String> lines = readFile("input16.txt");
		int fieldCount = 20;
		//Teil 1
		ArrayList<int[]> bounds = new ArrayList<>();
		ArrayList<Integer> values = new ArrayList<>();
		
		//Teil 2
		ArrayList<ArrayList<Integer>> tickets = new ArrayList<>();
		ArrayList<String> fieldNames = new ArrayList<>();
		HashMap<Integer, ArrayList<String>> fieldPoss = new HashMap<>();
		HashMap<String, int[]> fieldBounds = new HashMap<>();
		ArrayList<Integer> myTicket = new ArrayList<>();
		boolean myTicketFound = false;
		for(String line : lines) {
			if(line.contentEquals("") || line.contains("ticket"))
				continue;
			if(line.contains(":")) {
				int[] bound = new int[4];
				String fieldName = line.split(":")[0];
				String[] lineSplit = line.split(": ")[1].split(" or ");
				bound[0] = Integer.parseInt(lineSplit[0].split("-")[0]);
				bound[1] = Integer.parseInt(lineSplit[0].split("-")[1]);
				bound[2] = Integer.parseInt(lineSplit[1].split("-")[0]);
				bound[3] = Integer.parseInt(lineSplit[1].split("-")[1]);
				bounds.add(bound);
				
				fieldBounds.put(fieldName, bound);
				fieldNames.add(fieldName);
			}
			else {
				ArrayList<Integer> ticket = new ArrayList<Integer>();
				for(String value : line.split(",")) {
						values.add(Integer.parseInt(value));
						ticket.add(Integer.parseInt(value));
				}
				if(!myTicketFound) {
					myTicketFound = true;
					myTicket = new ArrayList<Integer>(ticket);
				}
				else {
					tickets.add(new ArrayList<Integer>(ticket));
				}
			}
		}
		
		/* Teil 1
		long scanError = 0;
		for(int value : values) {
			boolean correct = false;
			for(int[] bound : bounds) {
				if((value >= bound[0] && value <= bound[1]) || (value >= bound[2] && value <= bound[3])) {
					correct = true;
					break;
				}
			}
			if(!correct)
				scanError += value;
		}
		debug(scanError);
		*/
		
		//Teil 2
		for(int i = 0; i < fieldCount; i++) 
			fieldPoss.put(i, new ArrayList<String>(fieldNames));
		
		//filter invalid
		ArrayList<ArrayList<Integer>> ticketCopy = new ArrayList<>(tickets);
		for(ArrayList<Integer> ticket : tickets) {		
			for(int i = 0; i < fieldCount; i++) {
				boolean correct = false;
				for(int[] bound : bounds) {
					if((ticket.get(i) >= bound[0] && ticket.get(i) <= bound[1]) || (ticket.get(i) >= bound[2] && ticket.get(i) <= bound[3])) {
						correct = true;
						break;
					}
				}
				if(!correct)
					ticketCopy.remove(ticket);
			}
		}
		tickets = new ArrayList<>(ticketCopy);
		
		//reduce possibilities per fieldposition to hopefully one each
		for(ArrayList<Integer> ticket : tickets) {
			for(int i = 0; i < fieldCount; i++) {
				int value = ticket.get(i);
				ArrayList<String> poss = fieldPoss.get(i);
				boolean possible = true;
				ArrayList<String> possToRemove = new ArrayList<String>();
				for(String field : poss) {
					int[] currBounds = fieldBounds.get(field);
					if(!((value >= currBounds[0] && value <= currBounds[1]) || (value >= currBounds[2] && value <= currBounds[3]))) {
						possible = false;
						possToRemove.add(field);
					}
				}
				if(!possible) {
					poss.removeAll(possToRemove);
					fieldPoss.replace(i, poss);
				}
			}
		}
		ArrayList<String> fieldPossFound = new ArrayList<>();
		for(int j = 0; j < 20; j++) {
			for(int i = 0; i < fieldCount; i++) {
				ArrayList<String> poss = fieldPoss.get(i);
				if(poss.size() != 1) 
					poss.removeAll(fieldPossFound);
				if(poss.size() == 1) {
					fieldPossFound.add(poss.get(0));
				}
				fieldPoss.replace(i, poss);
			}	
		}
		
		long solution = 1;
		for(int key : fieldPoss.keySet()) {
			if(fieldPoss.get(key).get(0).contains("departure")) {
				solution *= myTicket.get(key);
			}
		}
		
		debug(fieldPoss);
		debug(myTicket);
		debug(solution);
	}
	
	static void Tag17() {
		ArrayList<String> lines = readFile("input17.txt");
		ArrayList<ArrayList<Integer>> activeCubes = new ArrayList<>();
		int x = 0, y = 0;
		for(String line : lines) {
			String[] points = line.split("");
			for(String point : points) {
				if(point.contentEquals("#")) {
					ArrayList<Integer> coords = new ArrayList<>();
					coords.add(x);
					coords.add(y);
					coords.add(0);
					coords.add(0);
					activeCubes.add(coords);
				}
				y++;
			}
			y = 0;
			x++;
		}
		debug(activeCubes);
		
		HashMap<ArrayList<Integer>, Integer> neighbors;
		for(int i = 0; i < 6; i++) {
			neighbors = new HashMap<>();
			for(ArrayList<Integer> coords : activeCubes) {
				for(int xOffset = -1; xOffset <= 1; xOffset++) {
					for(int yOffset = -1; yOffset <= 1; yOffset++) {
						for(int zOffset = -1; zOffset <= 1; zOffset++) {
							for(int wOffset = -1; wOffset <= 1; wOffset++) {
								if(!(xOffset == 0 && yOffset == 0 && zOffset == 0 && wOffset == 0)) {
									ArrayList<Integer> neighborCoords = new ArrayList<>();
									int neighborX = coords.get(0) + xOffset;
									int neighborY = coords.get(1) + yOffset;
									int neighborZ = coords.get(2) + zOffset;
									int neighborW = coords.get(3) + wOffset;
									neighborCoords.add(neighborX);
									neighborCoords.add(neighborY);
									neighborCoords.add(neighborZ);
									neighborCoords.add(neighborW);
									if(neighbors.containsKey(neighborCoords)) {
										int count = neighbors.get(neighborCoords);
										count++;
										neighbors.replace(neighborCoords, count);
									}
									else {
										neighbors.put(neighborCoords, 1);
									}
								}
							}
						}
					}
				}		
			}
			ArrayList<ArrayList<Integer>> newActiveCubes = new ArrayList<>();
			for(ArrayList<Integer> coords : neighbors.keySet()) {
				if(activeCubes.contains(coords)) {
					if(neighbors.get(coords) == 2 || neighbors.get(coords) == 3) {
						newActiveCubes.add(coords);
					}
				}
				else {
					if(neighbors.get(coords) == 3) {
						newActiveCubes.add(coords);
					}
				}				
			}
			activeCubes = new ArrayList<>(newActiveCubes);
		}
		debug(activeCubes.size());
	}
	
	static void Tag18() {
		ArrayList<String> lines = readFile("input18.txt");
		ArrayList<ArrayList<Character>> input = new ArrayList<>();
		for(String line : lines) {
			line = line.replace(" ", "");
			ArrayList<Character> chars = new ArrayList<>();
			for(Character c : line.toCharArray()) {
				chars.add(c);
			}
			input.add(chars);
		}
		
		long solution = 0;
		for(ArrayList<Character> expr : input) {
			solution += calculate(expr);
		}
		debug(solution);
	}
	
	static long calculate(ArrayList<Character> expr) {
		Deque<Character> deque = new ArrayDeque<>();
		long ret = 0;
		int parLevel = 0;
		ArrayList<Character> currentSubExpr = new ArrayList<>();
		for(Character c : expr) {
			if(parLevel > 0 || c == '(') {
				currentSubExpr.add(c);
			}
			else {
				deque.add(c);
			}
			
			if(c == '(') 
				parLevel++;
			else {
				if(c == ')') {
					parLevel--;
					if(parLevel == 0) {
						currentSubExpr.remove(0);
						currentSubExpr.remove(currentSubExpr.size() - 1);
						long result = calculate(currentSubExpr);
						char[] resultChars = Long.toString(result).toCharArray();
						for(Character retChar : resultChars)
							deque.add(retChar);	
						currentSubExpr = new ArrayList<>();
					} 
				}
			}		
		}
		
		//debug(deque);
		/* Teil 1
		String num = "";
		char op = '+';
		while(!deque.isEmpty()) {
			char c = deque.remove();
			if(Character.isDigit(c)) {
				num = num + c;
			}
			else {
				if(op == '+') {
					ret += Long.parseLong(num);
				}
				else {
					ret *= Long.parseLong(num);
				}
				op = c;
				num = "";
			}
		}
		if(op == '+') {
			ret += Long.parseLong(num);
		}
		else {
			ret *= Long.parseLong(num);
		}
		*/
		//Teil 2
		String calc = "";
		while(!deque.isEmpty())
			calc = calc + deque.remove();
		
		ret = 1;
		if(calc.contains("+") || calc.contains("*")) {
			String[] subCalcs = calc.split("\\*");
			for(String subExpr : subCalcs) {
				String num = "";
				long number = 0;
				for(int i = 0; i < subExpr.length(); i++) {
					if(Character.isDigit(subExpr.charAt(i))) {
						num = num + subExpr.charAt(i);
					}
					else {
						number += Long.parseLong(num);
						num = "";
					}	
				}
				number += Long.parseLong(num);
				ret *= number;
			}
		}
		
		return ret;
	}
	
	static void Tag19() {
		ArrayList<String> lines = readFile("input19.txt");
		HashMap<Integer, String> rules = new HashMap<>();
		int index = 0;
		for(String line : lines) {
			index++;
			if(line.length() == 0)
				break;
			String[] lineSplit = line.split(": ");
			Integer ruleNo = Integer.parseInt(lineSplit[0]);
			String rule = lineSplit[1];
			rule = rule.replace("\"", "");
			rules.put(ruleNo, rule);
		}
		
		
		String regex = getRegex(rules, 0);
		int count = 0;
		for(int i = index; i < lines.size(); i++) {
			if(lines.get(i).matches(regex)) {
				count++;
			}
		}

		debug(count);
		
		//Teil 2
		String reg42 = getRegex(rules, 42);
		String reg31 = getRegex(rules, 31);
		ArrayList<String> poss = new ArrayList<>();
		for(int count8 = 1; count8 < 10; count8++) {
			for(int count11 = 1; count11 < 10; count11++) {
				String newPoss = "(" + reg42 + ")" + "{" + count8 + "}" + "(" + reg42 + ")" + "{" + count11 + "}" + "(" + reg31 + ")" + "{" + count11 + "}";
				poss.add(newPoss);
			}
		}
		
		int count2 = 0;
		for(int i = index; i < lines.size(); i++) {
			for(String currPoss: poss) {
				if(lines.get(i).matches(currPoss)) {
					count2++;
					break;
				}
			}
		}
		debug(count2);
	}
	
	static String getRegex(HashMap<Integer, String> rules, int nr) {
		String regex = rules.get(nr);
		while(regex.matches(".*\\d.*")){
			String[] regexSplit = regex.split(" ");
			regex = "";
			for(String group : regexSplit) {
				if(group.matches(".*\\d.*")) {
					int num = Integer.parseInt(group);
					regex = regex + "( " + rules.get(num) + " )";
				}
				else {
					regex = regex + group;
				}
			}
		}
		regex = regex.replace(" ", "");
		return regex;
	}
	
	static void Tag21() {
		ArrayList<String> lines = readFile("input21.txt");
		HashMap<String, Set<String>> contained = new HashMap<>();
		Set<String> ings = new HashSet<>();
		for(String line : lines) {
			String[] split = line.split(" \\(contains ");
			Set<String> ingredients = new HashSet<>();
			for(String ing : split[0].split(" ")) {
				ingredients.add(ing);
			}
			ings.addAll(ingredients);
			split[1] = split[1].replace(")", "");
			for(String all : split[1].split(", ")) {
				Set<String> ingredientsCopy = new HashSet<>(ingredients);
				if(contained.containsKey(all)) {
					Set<String> ingList = contained.get(all);
					ingList.retainAll(ingredients);
					contained.replace(all, ingList);
				}
				else {
					contained.put(all, ingredientsCopy);
				}
			}
		}
		Set<String> ingPossFound = new HashSet<>();
		for(int j = 0; j < 20; j++) {
			for(String key : contained.keySet()) {
				Set<String> poss = contained.get(key);
				if(poss.size() != 1) 
					poss.removeAll(ingPossFound);
				if(poss.size() == 1) {
					ingPossFound.add((String)poss.toArray()[0]);
				}
				contained.replace(key, poss);
			}	
		}
		ings.removeAll(ingPossFound);
		int count = 0;
		for(String ing : ings) {
			for(String line : lines) {
				if(line.split(" \\(contains ")[0].matches(".*\\b"+ing+"\\b.*"))
					count++;
			}
		}
		debug(count);
		
		//Teil 2
		ArrayList<String> foundIngs = new ArrayList<>(contained.keySet());
		foundIngs.sort(null);
		String output = "";
		for(String ing : foundIngs) {
			output = output + contained.get(ing).toArray()[0] + ",";
		}
		output = output.substring(0, output.length() - 1);
		debug(contained);
		debug(foundIngs);
		debug(output);
	}
	
	static void Tag22() {
		ArrayList<String> lines = readFile("input22.txt");
		Deque<Integer> p1Deck = new ArrayDeque<>();
		Deque<Integer> p2Deck = new ArrayDeque<>();
		boolean atP2 = false;
		for(String line : lines) {
			if(line.contains("Player 1"))
				continue;
			if(line.contains("Player 2")) {
				atP2 = true;
				continue;
			}
			if(line.contentEquals(""))
				continue;
			if(atP2) {
				p2Deck.add(Integer.parseInt(line));
			}
			else {
				p1Deck.add(Integer.parseInt(line));
			}
		}

		/*
		while(!(p1Deck.isEmpty() || p2Deck.isEmpty())) {
			int p1Card = p1Deck.poll();
			int p2Card = p2Deck.poll();
			if(p1Card > p2Card) {
				p1Deck.add(p1Card);
				p1Deck.add(p2Card);
			}
			else {
				p2Deck.add(p2Card);
				p2Deck.add(p1Card);
			}
		}*/
		
		//Teil 2
		int gameWinner = 0;
		while(!(p1Deck.isEmpty() || p2Deck.isEmpty())) {
			int p1Card = p1Deck.poll();
			int p2Card = p2Deck.poll();
			int winner = 0;
			if(p1Card <= p1Deck.size() && p2Card <= p2Deck.size()) {
				Deque<Integer> p1SubDeck = new ArrayDeque<>(p1Deck);
				Deque<Integer> p2SubDeck = new ArrayDeque<>(p2Deck);
				while(p1SubDeck.size() > p1Card)
					p1SubDeck.removeLast();
				while(p2SubDeck.size() > p2Card)
					p2SubDeck.removeLast();
				winner = playGame(p1SubDeck, p2SubDeck);
			}
			else {
				winner = p1Card > p2Card ? 1 : 2;
			}
			if(winner == 1) {
				p1Deck.add(p1Card);
				p1Deck.add(p2Card);
			}
			else {
				p2Deck.add(p2Card);
				p2Deck.add(p1Card);
			}
		}
		gameWinner = p1Deck.isEmpty() ? 2 : 1;
			
		int count = 0;
		if(gameWinner == 2) {
			int i = p2Deck.size();
			for(int card : p2Deck) {
				count += card * i;
				i--;
			}
		}
		else {
			int i = p1Deck.size();
			for(int card : p1Deck) {
				count += card * i;
				i--;
			}
		}
		debug(count);
	}

	static int playGame(Deque<Integer> p1Deck, Deque<Integer> p2Deck) {
		ArrayList<String> p1Prev = new ArrayList<String>();
		ArrayList<String> p2Prev = new ArrayList<String>();
		while(!(p1Deck.isEmpty() || p2Deck.isEmpty())) {
			if(p1Prev.contains(deckToString(p1Deck)) && p2Prev.contains(deckToString(p2Deck))) {
				return 1;
			}
			p1Prev.add(deckToString(p1Deck));
			p2Prev.add(deckToString(p2Deck));
			int p1Card = p1Deck.poll();
			int p2Card = p2Deck.poll();
			int winner = 0;
			if(p1Card <= p1Deck.size() && p2Card <= p2Deck.size()) {
				Deque<Integer> p1SubDeck = new ArrayDeque<>(p1Deck);
				Deque<Integer> p2SubDeck = new ArrayDeque<>(p2Deck);
				while(p1SubDeck.size() > p1Card)
					p1SubDeck.removeLast();
				while(p2SubDeck.size() > p2Card)
					p2SubDeck.removeLast();
				winner = playGame(p1SubDeck, p2SubDeck);
			}
			else {
				winner = p1Card > p2Card ? 1 : 2;
			}
			if(winner == 1) {
				p1Deck.add(p1Card);
				p1Deck.add(p2Card);
			}
			else {
				p2Deck.add(p2Card);
				p2Deck.add(p1Card);
			}
		}
		return p1Deck.isEmpty() ? 2 : 1;
	}
	
	static String deckToString (Deque<Integer> deck) {
		String ret = "";
		for(int card : deck) {
			ret = ret + "-" + card;
		}
		return ret;
	}
	
	static void Tag23() {
		ArrayList<Integer> cups = new ArrayList<>();
		String line = "962713854";
		/*
		for(int i = 0; i < line.length(); i++) {
			cups.add(Integer.parseInt(line.substring(i,i+1)));
		}

		for(int i = 0; i < 10000000; i++) {
			int cup1 = cups.remove(1);
			int cup2 = cups.remove(1);
			int cup3 = cups.remove(1);
			int dest = cups.get(0) - 1;
			while(!cups.contains(dest)) {
				dest--;
				if(dest < 1)
					dest = 9;
			}
			int destIdx = cups.indexOf(dest);
			cups.add(destIdx + 1, cup1);
			cups.add(destIdx + 2, cup2);
			cups.add(destIdx + 3, cup3);
			int shiftedCup = cups.remove(0);
			cups.add(shiftedCup);
		}
		debug(cups);*/
		
		//Teil 2
		HashMap<Integer, Integer> cupList = new HashMap<>();
		cupList.put(1000000,9);
		for(int i = 0; i < 8; i++) {
			cupList.put(Integer.parseInt(line.substring(i,i+1)), Integer.parseInt(line.substring(i + 1, i + 2)));
		}
		cupList.put(4, 10);
		for(int i = 10; i < 1000000; i++) {
			cupList.put(i,i+1);
		}
		//moves
		int currentCup = 9;
		for(int i = 0; i < 10000000; i++) {
			int cup1 = cupList.get(currentCup);
			int cup2 = cupList.get(cup1);
			int cup3 = cupList.get(cup2);
			int cup4 = cupList.get(cup3);
			cupList.remove(cup1);
			cupList.remove(cup2);
			cupList.remove(cup3);
			cupList.replace(currentCup, cup4);
			int dest = currentCup - 1;
			while(!cupList.containsKey(dest)) {
				dest--;
				if(dest < 1) 
					dest = 1000000;
			}
			int destNext = cupList.get(dest);
			cupList.replace(dest, cup1);
			cupList.put(cup1, cup2);
			cupList.put(cup2, cup3);
			cupList.put(cup3, destNext);
			currentCup = cupList.get(currentCup);
		}
		int c1 = cupList.get(1);
		int c2 = cupList.get(c1);
		long sol = (long)c1 * c2;
		debug(c1);
		debug(c2);
		debug(sol);
}
	
	static void Tag24() {
		ArrayList<String> lines = readFile("input24.txt");
		ArrayList<ArrayList<Integer>> flippedTiles = new ArrayList<>();
		for(String line : lines) {
			ArrayList<Integer> coords = StringToCoords(line);
			if(flippedTiles.contains(coords))
				flippedTiles.remove(coords);
			else
				flippedTiles.add(coords);
		}
		debug(flippedTiles.size());
		
		HashMap<ArrayList<Integer>, Integer> neighbors;
		int[][] neighborOffsets = {{2,0}, {-2,0}, {1,1}, {-1,1}, {1,-1}, {-1,-1}};
		for(int i = 0; i < 100; i++) {
			neighbors = new HashMap<>();
			for(ArrayList<Integer> coords : flippedTiles) {
				for(int[] offset : neighborOffsets) {
					ArrayList<Integer> neighborCoords = new ArrayList<>();
					int x = coords.get(0) + offset[0];
					int y = coords.get(1) + offset[1];
					neighborCoords.add(x);
					neighborCoords.add(y);
					if(neighbors.containsKey(neighborCoords)) {
						int count = neighbors.get(neighborCoords);
						count++;
						neighbors.replace(neighborCoords, count);
					}
					else {
						neighbors.put(neighborCoords, 1);						
					}
				}
				
			}
			
			ArrayList<ArrayList<Integer>> newFlippedTiles = new ArrayList<>(flippedTiles);
			for(ArrayList<Integer> coords : flippedTiles) {
				if(neighbors.containsKey(coords)) {
					if(neighbors.get(coords) > 2) 
						newFlippedTiles.remove(coords);
				}
				else {
					newFlippedTiles.remove(coords);
				}
			}
			for(ArrayList<Integer> coords : neighbors.keySet()) {
				if(!flippedTiles.contains(coords)) {
					if(neighbors.get(coords) == 2)
						newFlippedTiles.add(coords);
				}
			}
			flippedTiles = new ArrayList<>(newFlippedTiles);
		}
		debug(flippedTiles.size());
	}
	
	static ArrayList<Integer> StringToCoords(String line) {
		ArrayList<Integer> coords = new ArrayList<>();
		int e = 0, w = 0, ne = 0, nw = 0, se = 0, sw = 0;
		while (line.length() != 0) {
			switch (line.charAt(0)) {
			case 'e': e++; line = line.substring(1); break;
			case 'w': w++; line = line.substring(1); break;
			case 'n': if(line.charAt(1) == 'e') ne++; else nw++; line = line.substring(2);break;
			case 's': if(line.charAt(1) == 'e') se++; else sw++; line = line.substring(2); break;
			}
		}
		
		int x = 2 * e + ne + se - 2 * w - nw - sw;
		int y = ne + nw - se - sw;
		
		/*
		 * OXOXOXOXOXOXOXOXOXOX		   
		 * XOXOXOXOXOXOXOXOXOXO	x->  y ^  
		 * OXOXOXOXOXOXOXOXOXOX
		*/
		coords.add(x);
		coords.add(y);
		return coords;
	}
	
	static void Tag25() {
		long doorPubKey = 335121;
		long cardPubKey = 363891;
		
		int doorLoopSize = 0;
		long value = 1;
		long subjectNumber = 7;
		while (value != doorPubKey) {
			doorLoopSize++;
			value *= subjectNumber;
			value = value % 20201227;
		}
		long sol = transformNumber(cardPubKey, doorLoopSize);
		debug(doorLoopSize);
		debug(value);
		debug(sol);
	}
	
	static long transformNumber (long subjectNumber, long loopSize) {
		long value = 1;
		for(int i = 0; i < loopSize; i++) {
			value *= subjectNumber;
			value = value % 20201227;
		}
		return value;
	}
	
	static ArrayList<String> readFile (String name) {
		ArrayList<String> lines = new ArrayList<String>();
		BufferedReader reader;
		try {
			reader = new BufferedReader(new FileReader("input/" + name));
			String line;
			boolean eof = false;
			while(!eof) {
				line = reader.readLine();
				if(line == null) {
					eof = true;
					break;
				}
				lines.add(line);
			}
			reader.close();
		}
		catch (Exception e) {
			e.printStackTrace();
		}
		return lines;
	}
	
	static void debug(Object o) {
		System.out.println(o);
	}
	
}
