mport json
import requests
from bs4 import BeautifulSoup
def get_usernames(username):
 results = {
 "Chess.com": check_chess_username(username),
 "Codeacademy.com": check_codeacademy_username(username),
 "Codechef.com": check_codechef_username(username),
 "Codewars.com": check_codewars_username(username),
 "Leetcode.com": check_leetcode_username(username)
 }
 result_string = "\n".join(f"{platform}: {'yes' if is_valid else 'no'}" for platform, is_valid in results.items())
 print(result_string)
 return result_string
def check_chess_username(username):
 url = f"https://www.chess.com/callback/user/valid?username={username}"
 response = requests.get(url)
 data = json.loads(response.text)
 messages = data["messages"]
 if "Username is valid" in messages:
 return False
 else:
 return True
def check_codeacademy_username(username):
 try:
 session = requests.Session()
 headers = {
 'User-Agent': 'Mozilla/5.0 (Windows NT 10.0;Win64) AppleWebKit/537.36 (KHTML, like Gecko) '
 'Chrome/58.0.3029.110 Safari/537.3'}
 url = f"https://www.codecademy.com/profiles/{username}"
 response = session.get(url, headers=headers)
 soup = BeautifulSoup(response.content, 'html.parser')
 error_message_element = soup.find("h2", class_="gamut-1mjim47-Text e8i0p5k0")
 if error_message_element is not None:
 error_message = error_message_element.get_text()
 if "This profile could not be found" in error_message:
 return False
 else:
 return True
 except requests.RequestException as e:
 print(f"An error occurred: {e}")
def check_codechef_username(username):
 url = f"https://www.codechef.com/users/{username}"
 response = requests.get(url)
 soup = BeautifulSoup(response.content, "html.parser")
 if soup.find("div", class_="user-profile-container") is not None:
 return True
 else:
 return False
def check_codewars_username(username):
 url = f"https://www.codewars.com/users/{username}"
 response = requests.get(url)
 if response.status_code == 200:
 return True
 else:
 return False
def check_leetcode_username(username):
 url = f"https://leetcode.com/{username}"
 response = requests.get(url)
 if response.status_code == 404:
 return False
 else:
 return True